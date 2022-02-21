//! LOKI fuzzing engine

use crate::loki_message::LokiMessage;
use crate::message_pool::MessagePool;
use crate::state_model::StateModel;
use anyhow::Result;

// define the callback type
type CallBack = fn();

/// LOKI fuzz engine struct
pub struct Engine {
    /// the message pool of the engine
    message_pool: MessagePool,
    /// connected nodes of loki node
    connnected_nodes: Vec<String>,
    /// state model of the tested protocol
    state_model: StateModel,

    // some callback that need to be implementd by the user
    /// signing callback function
    sign_call_back: CallBack,
    /// packets sending callback function
    send_call_back: CallBack,
}

impl Engine {
    /// passive sending fuzz packets without initial seeds
    /// this function returns the next message that the fuzzer need to send with LokiMessage type
    pub fn passive_sending(&mut self) -> Result<Option<LokiMessage>> {
        todo!()
        // return Ok(Some(LokiMessage)) or Ok(None) or Err()
    }

    /// passive sending fuzz packets with initial seeds
    pub fn passive_sending_with_seeds(
        &mut self,
        _initial_seeds: Vec<LokiMessage>,
    ) -> Result<Option<LokiMessage>> {
        todo!()
    }

    /// active sending fuzz packets
    pub fn active_sending(&mut self, _node_addrs: Vec<String>) -> Result<bool> {
        // (self.send_call_back)();
        todo!()
    }

    /// set the addresses of each node
    pub fn set_node_addr(&mut self, _node_addrs: Vec<String>) -> Result<bool> {
        self.connnected_nodes = _node_addrs;
        Ok(true)
    }

    /// get the connnected nodes
    pub fn get_connected_nodes(&self) -> Result<Vec<String>> {
        Ok(self.connnected_nodes.clone())
    }

    /// get the mutable connected nodes
    pub fn get_mut_connected_nodes(&mut self) -> Result<&mut Vec<String>> {
        Ok(&mut self.connnected_nodes)
    }

    /// add received messages to the message pool
    pub fn add_to_message_pool(&mut self, _message: Vec<LokiMessage>) -> Result<bool> {
        todo!()
    }

    /// init the fuzz engine, the main controller of the LOKI
    /// First, the engine resisters the callback functions
    /// Then, the engine inits the scepter parser
    /// After that, the engine constructs the state model
    /// Then, the engine start the active sending thread and start fuzz
    pub fn new_fuzz_engine(&mut self) -> Result<()> {
        todo!()
    }

    // some protected functions that can only be called in the engine object
    /// set the packet sending callback function
    #[allow(dead_code)]
    fn set_send_callback(&mut self, f: CallBack) -> Result<bool> {
        self.send_call_back = f;
        Ok(true)
    }

    /// set the signing callback function
    #[allow(dead_code)]
    fn set_sign_callback(&mut self, f: CallBack) -> Result<bool> {
        self.sign_call_back = f;
        Ok(true)
    }

    /// set the message_pool
    #[allow(dead_code)]
    fn set_message_pool(&mut self, new_pool: MessagePool) -> Result<bool> {
        self.message_pool = new_pool;
        Ok(true)
    }

    /// get the message_pool
    #[allow(dead_code)]
    fn get_message_pool(&self) -> Result<MessagePool> {
        Ok(self.message_pool.clone())
    }

    /// get the mutable message_pool
    #[allow(dead_code)]
    fn get_mut_message_pool(&mut self) -> Result<&mut MessagePool> {
        Ok(&mut self.message_pool)
    }

    /// set the state model
    #[allow(dead_code)]
    fn set_state_model(&mut self, new_model: StateModel) -> Result<bool> {
        self.state_model = new_model;
        Ok(true)
    }

    /// get the state model
    #[allow(dead_code)]
    fn get_state_model(&self) -> Result<StateModel> {
        Ok(self.state_model.clone())
    }

    /// get the mutable state model
    #[allow(dead_code)]
    fn get_mut_state_model(&mut self) -> Result<&mut StateModel> {
        Ok(&mut self.state_model)
    }
}
