//! The interface that user needs to extend and implement for LOKI adaption
use crate::loki_message::LokiMessage;
use serde_json::{Map, Value};

/// the adatption interface
/// encode the LOKI message into a stream
pub fn encode(_msg_name: String, _content: Map<String, Value>) -> Vec<u8> {
    todo!();
}

/// decode a stream to a LOKI message
pub fn decode(_msg_name: String, _stream: &[u8]) -> Map<String, Value> {
    todo!();
}

/// the package sending interface of tested systems
pub fn send_packets(_target_id: String, _msg: LokiMessage) {
    todo!();
}

/// receive the packets
pub fn recv_packets() {}
