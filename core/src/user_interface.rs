//! The interface that user needs to extend and implement for LOKI adaption
use crate::loki_message::LokiMessage;

/// the adatption interface
/// unwrap the LokiMessage into developer defined message type
pub fn unwrap() -> LokiMessage {
    todo!();
}

/// wrap the user defined message type into LokiMessage
pub fn wrap() {
    todo!();
}

/// the package sending interface of tested systems
pub fn send_packets(_target_id: String, _msg: LokiMessage) {
    todo!();
}

/// receive the packets
pub fn recv_packets() {}

/// init the fuzzer
pub fn init_fuzz() {}

/// the interface for signing
pub fn sign() {
    todo!();
}
