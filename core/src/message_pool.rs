//! The message pool of Loki fuzz engine

use crate::loki_message::LokiMessage;
use anyhow::Result;

/// The message pool struct
#[derive(Debug, Clone)]
pub struct MessagePool {
    /// the messages in the pool
    _messages: Vec<LokiMessage>,
}

impl MessagePool {
    /// construct a new message pool
    pub fn new(_messages: Vec<LokiMessage>) -> Self {
        Self { _messages }
    }

    /// add received messages to the message pool
    pub fn add_msgs_to_pool(&mut self, _message: Vec<LokiMessage>) -> Result<bool> {
        todo!()
    }

    /// add one message to the message pool
    pub fn add_msg_to_pool(&mut self, _message: LokiMessage) -> Result<bool> {
        todo!()
    }

    /// delete from the messages pool
    pub fn delete_from_messasge_pool(&mut self) -> Result<bool> {
        todo!()
    }

    /// find the latest message of certain type
    pub fn find_latest_message_with_type(&self, _msg_type: String) -> Result<LokiMessage> {
        todo!()
    }
}
