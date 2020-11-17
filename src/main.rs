//! This is a modified version of the https://github.com/stm32-rs/stm32-eth/blob/master/examples/ip.rs example.

#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_abort;

use cortex_m::{asm, interrupt::Mutex};
use cortex_m_rt::{entry, exception};
use stm32_eth::{
    hal::gpio::GpioExt,
    hal::prelude::*,
    hal::rcc::RccExt,
    hal::time::U32Ext,
    stm32::{interrupt, CorePeripherals, Peripherals, SYST},
    Eth, EthPins, PhyAddress, RingEntry,
};

use core::cell::RefCell;
use core::fmt::Write;
use core::mem::MaybeUninit;

use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache};
use smoltcp::socket::{SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};

use modality_probe::{
    initialize_at, record, record_w_u16, ModalityProbe, NanosecondResolution, Probe,
    RestartCounterProvider, WallClockId,
};

// Import the generated component manifest definitions
mod component_definitions;
use component_definitions::*;

/// Global and `no_mangle` so we can easily resolve the symbol
/// with modality-probe-debug-collector
#[no_mangle]
static mut PROBE_BUFFER: [MaybeUninit<u8>; 1024] = [MaybeUninit::new(0u8); 1024];

const SRC_MAC: [u8; 6] = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
const IP_ADDR: [u8; 4] = [192, 168, 200, 100];
const LISTEN_PORT: u16 = 80;

static TIME: Mutex<RefCell<u64>> = Mutex::new(RefCell::new(0));
static ETH_PENDING: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[entry]
fn main() -> ! {
    let probe = unsafe {
        initialize_at!(
            &mut PROBE_BUFFER,
            EXAMPLE_PROBE,
            NanosecondResolution::UNSPECIFIED,
            WallClockId::LOCAL_ONLY,
            RestartCounterProvider::NoRestartTracking,
            tags!("example", "ip"),
            "Example probe"
        )
    }
    .expect("Could not initialize probe");

    let p = Peripherals::take().unwrap();
    let mut cp = CorePeripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    // HCLK must be at least 25MHz to use the ethernet peripheral
    let clocks = rcc.cfgr.sysclk(32.mhz()).hclk(32.mhz()).freeze();

    setup_systick(&mut cp.SYST);

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let gpiog = p.GPIOG.split();

    let mut led = gpiob.pb7.into_push_pull_output();

    let eth_pins = EthPins {
        ref_clk: gpioa.pa1,
        md_io: gpioa.pa2,
        md_clk: gpioc.pc1,
        crs: gpioa.pa7,
        tx_en: gpiog.pg11,
        tx_d0: gpiog.pg13,
        tx_d1: gpiob.pb13,
        rx_d0: gpioc.pc4,
        rx_d1: gpioc.pc5,
    };

    let mut rx_ring: [RingEntry<_>; 8] = Default::default();
    let mut tx_ring: [RingEntry<_>; 2] = Default::default();
    let mut eth = Eth::new(
        p.ETHERNET_MAC,
        p.ETHERNET_DMA,
        &mut rx_ring[..],
        &mut tx_ring[..],
        PhyAddress::_0,
        clocks,
        eth_pins,
    )
    .unwrap();
    eth.enable_interrupt();

    let local_addr = Ipv4Address::from_bytes(&IP_ADDR);
    let ip_addr = IpCidr::new(IpAddress::from(local_addr), 24);
    let mut ip_addrs = [ip_addr];
    let mut neighbor_storage = [None; 16];
    let neighbor_cache = NeighborCache::new(&mut neighbor_storage[..]);
    let ethernet_addr = EthernetAddress(SRC_MAC);
    let mut iface = EthernetInterfaceBuilder::new(&mut eth)
        .ethernet_addr(ethernet_addr)
        .ip_addrs(&mut ip_addrs[..])
        .neighbor_cache(neighbor_cache)
        .finalize();

    let mut server_rx_buffer = [0; 2048];
    let mut server_tx_buffer = [0; 2048];
    let server_socket = TcpSocket::new(
        TcpSocketBuffer::new(&mut server_rx_buffer[..]),
        TcpSocketBuffer::new(&mut server_tx_buffer[..]),
    );
    let mut sockets_storage = [None, None];
    let mut sockets = SocketSet::new(&mut sockets_storage[..]);
    let server_handle = sockets.add(server_socket);

    record!(
        probe,
        IP_STACK_INITIALIZED,
        "TCP/IP stack initialized",
        tags!("ip")
    );

    loop {
        let time: u64 = cortex_m::interrupt::free(|cs| *TIME.borrow(cs).borrow());

        cortex_m::interrupt::free(|cs| {
            let mut eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
            *eth_pending = false;
        });

        match iface.poll(&mut sockets, Instant::from_millis(time as i64)) {
            Ok(true) => {
                led.toggle().unwrap();

                record!(
                    probe,
                    IP_STACK_STATE_CHANGE,
                    "IP stack had a state change",
                    tags!("ip")
                );

                let mut socket = sockets.get::<TcpSocket>(server_handle);
                if !socket.is_open() {
                    socket.listen(LISTEN_PORT).unwrap();

                    record_w_u16!(
                        probe,
                        SOCKET_LISTENING,
                        LISTEN_PORT,
                        tags!("socket", "listen"),
                        "Socket listening"
                    );
                }

                if socket.can_send() {
                    writeln!(socket, "hello")
                        .map(|_| {
                            socket.close();
                        })
                        .unwrap();

                    record!(
                        probe,
                        SENT_A_MESSAGE,
                        "Sent a message",
                        tags!("socket", "message")
                    );
                }
            }
            Ok(false) => {
                // Sleep if no ethernet work is pending
                cortex_m::interrupt::free(|cs| {
                    let eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
                    if !*eth_pending {
                        asm::wfi();
                        // Awaken by interrupt
                    }
                });
            }
            Err(_e) =>
            // Ignore malformed packets
            {
                record!(
                    probe,
                    MALFORMED_PACKET,
                    "Received a malformed or unknown packet",
                    tags!("ip")
                );
            }
        }
    }
}

fn setup_systick(syst: &mut SYST) {
    syst.set_reload(SYST::get_ticks_per_10ms() / 10);
    syst.enable_counter();
    syst.enable_interrupt();
}

#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        let mut time = TIME.borrow(cs).borrow_mut();
        *time += 1;
    })
}

#[interrupt]
fn ETH() {
    cortex_m::interrupt::free(|cs| {
        let mut eth_pending = ETH_PENDING.borrow(cs).borrow_mut();
        *eth_pending = true;
    });

    // Clear interrupt flags
    let p = unsafe { Peripherals::steal() };
    stm32_eth::eth_interrupt_handler(&p.ETHERNET_DMA);
}
