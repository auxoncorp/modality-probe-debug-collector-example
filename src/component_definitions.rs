/*
 * GENERATED CODE, DO NOT EDIT
 *
 * Component:
 *   Name: example-component
 *   ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
 *   Code hash: 1cf5e574c240fc37a41738ee59681a0773a6d29b4b7c7d290a89f1ef6e346c8a
 *   Instrumentation hash: 6759b872935a7b5701bc8471e1c58265463ed41855820a58ce2cccc5200acfe8
 */
use modality_probe::{EventId, ProbeId};

/*
 * Probes (sha3-256 e1fa6933db536a50c61fdfd804d7fc4cca677092504852c1fc01418eb424b5b9)
 */

/// Name: EXAMPLE_PROBE
/// Description: Example probe
/// Component ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
/// Tags: example;ip
/// Location: main.rs:53
pub const EXAMPLE_PROBE: ProbeId = unsafe { ProbeId::new_unchecked(118528911) };

/*
 * Events (sha3-256 67595003a8b4646ae889ab2cb2689687eeba6458575f415405fa68ca0da43ecf)
 */

/// Name: IP_STACK_INITIALIZED
/// Description: TCP/IP stack initialized
/// Component ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
/// Tags: ip
/// Payload type:
/// Location: main.rs:129
pub const IP_STACK_INITIALIZED: EventId = unsafe { EventId::new_unchecked(1) };

/// Name: IP_STACK_STATE_CHANGE
/// Description: IP stack had a state change
/// Component ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
/// Tags: ip
/// Payload type:
/// Location: main.rs:148
pub const IP_STACK_STATE_CHANGE: EventId = unsafe { EventId::new_unchecked(2) };

/// Name: SOCKET_LISTENING
/// Description: Socket listening
/// Component ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
/// Tags: socket;listen
/// Payload type: u16
/// Location: main.rs:159
pub const SOCKET_LISTENING: EventId = unsafe { EventId::new_unchecked(3) };

/// Name: SENT_A_MESSAGE
/// Description: Sent a message
/// Component ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
/// Tags: socket;message
/// Payload type:
/// Location: main.rs:175
pub const SENT_A_MESSAGE: EventId = unsafe { EventId::new_unchecked(4) };

/// Name: MALFORMED_PACKET
/// Description: Received a malformed or unknown packet
/// Component ID: 8a0774ae-4cba-4984-9029-7d1d7eeaac62
/// Tags: ip
/// Payload type:
/// Location: main.rs:196
pub const MALFORMED_PACKET: EventId = unsafe { EventId::new_unchecked(5) };
