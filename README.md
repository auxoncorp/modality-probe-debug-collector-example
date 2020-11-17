# Modality Probe Debug Collector Example

## Overview

A version of the [ip example](https://github.com/stm32-rs/stm32-eth/blob/4d6b29bf1ecdd1f68e5bc304a3d4f170049896c8/examples/ip.rs) from the [stm32-eth](https://crates.io/crates/stm32-eth) crate
with [modality-probe](https://github.com/auxoncorp/modality-probe) instrumentation.

See the [modality-probe-debug-collector README](https://github.com/auxoncorp/modality-probe/blob/master/collectors/modality-probe-debug-collector/README.md) for more information.

## Getting Started

### Dependencies

* [Rust Toolchain](https://rustup.rs)
* [OpenOCD](http://openocd.org/)

### Building

* Install system package dependencies:
    ```shell
    $ sudo apt install openocd
    ```
* Install the `thumbv7em-none-eabihf` Rust target:
    ```shell
    $ rustup update
    $ rustup target add thumbv7em-none-eabihf
    ```
* Build the example using `cargo`:
    ```shell
    $ git clone https://github.com/auxoncorp/modality-probe-debug-collector-example.git
    $ cd modality-probe-debug-collector-example/
    $ cargo build
    ```

## Usage

* Upload the ELF file to the device using `OpenOCD`:
    ```shell
    $ openocd \
        -f openocd.cfg \
        -c init \
        -c "reset halt" \
        -c "flash write_image erase target/thumbv7em-none-eabihf/debug/example-project" \
        -c "reset run" \
        -c "shutdown"
    ```
* You should be able to ping the device at `192.168.200.100`:
    ```shell
    $ ping 192.168.200.100
    PING 192.168.200.100 (192.168.200.100) 56(84) bytes of data.
    64 bytes from 192.168.200.100: icmp_seq=1 ttl=64 time=4.51 ms
    64 bytes from 192.168.200.100: icmp_seq=2 ttl=64 time=4.47 ms
    64 bytes from 192.168.200.100: icmp_seq=3 ttl=64 time=4.42 ms
    ```
* You should also be able to connect to the TCP socket on port `80`:
    ```shell
    $ netcat 192.168.200.100 80
    hello
    ```

## License

See [LICENSE](./LICENSE) for more details.

[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)
