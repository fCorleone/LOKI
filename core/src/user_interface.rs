//! The interface that user needs to extend and implement for LOKI adaption
use crate::loki_message::LokiMessage;

/// the adatption interface 
pub trait Adatption{
    /// unwrap the LokiMessage into developer defined message type
    fn unwrap() -> LokiMessage{
        todo!();
    }

    /// wrap the user defined message type into LokiMessage
    fn wrap(){
        todo!();
    }

    /// the package sending interface of tested systems
    fn send_packets(){
        todo!();
    }

    /// the interface for signing
    fn sign() {
        todo!();
    }

 }