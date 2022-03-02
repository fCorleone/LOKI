//! The normalization message representation in LOKI

use anyhow::Result;
use json::JsonValue;

/// the message structure in LOKI
#[derive(Debug, Clone)]
/// the loki message struct
pub struct LokiMessage {
    /// the nodeid who sends the message
    from: String,
    /// the content of the message, represented as a json object
    /// use json::JsonValue::new_object() to create an empty json object
    content: JsonValue,
    /// the type of the message
    msg_type: String,
}

impl LokiMessage {
    /// construct a new message with content
    pub fn new(from: String, content: JsonValue, msg_type: String) -> Self {
        Self {
            from,
            content,
            msg_type,
        }
    }

    /// construct a new message with only source node
    pub fn new_with_from(from: String) -> Self {
        let content = JsonValue::new_object();
        let msg_type = "".to_string();
        Self {
            from,
            content,
            msg_type,
        }
    }

    /// set the message type
    pub fn set_msg_type(&mut self, msg_type: String) -> Result<bool> {
        self.msg_type = msg_type;
        Ok(true)
    }

    /// get the message type
    pub fn get_msg_type(&self) -> String {
        self.msg_type.clone()
    }

    /// get the mutable message type
    pub fn get_mut_msg_type(&mut self) -> &mut String {
        &mut self.msg_type
    }

    /// set the from neighbour of a message
    pub fn set_from_node(&mut self, from_node: String) -> Result<bool> {
        self.from = from_node;
        Ok(true)
    }

    /// get the from neighbour of a message
    pub fn get_from_node(&self) -> Result<String> {
        Ok(self.from.clone())
    }

    /// get mutable from neighbour of a message
    pub fn get_mut_from_node(&mut self) -> Result<&mut String> {
        Ok(&mut self.from)
    }

    /// set the content of the message
    pub fn set_content(&mut self, new_content: JsonValue) -> Result<bool> {
        self.content = new_content;
        Ok(true)
    }

    /// get the content of the message
    pub fn get_content(&self) -> Result<JsonValue> {
        Ok(self.content.clone())
    }

    /// get the mutable content of the message
    pub fn get_mut_content(&mut self) -> Result<&mut JsonValue> {
        Ok(&mut self.content)
    }

    /// mutate the current message
    pub fn mutate(&mut self) -> LokiMessage {
        todo!()
    }

    /// generate a new message of certain type
    pub fn generate(_msg_type: String) -> Self {
        todo!()
    }
}
