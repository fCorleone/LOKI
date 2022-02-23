//! LOKI fuzzing engine

use crate::loki_message::LokiMessage;
use crate::message_pool::MessagePool;
use crate::neighbour::Neighbour;
use crate::state_model::{State, StateModel};
use anyhow::Result;

// define the callback type
type CallBack = fn();

/// LOKI fuzz engine struct
pub struct Engine {
    /// the message pool of the engine
    message_pool: MessagePool,
    /// connected nodes of loki node
    connnected_nodes: Vec<Neighbour>,
    /// state model of the tested protocol
    state_model: StateModel,
    /// current state of LOKI node
    cur_state: State,

    // some callback that need to be implementd by the user
    /// signing callback function
    sign_call_back: CallBack,
    /// packets sending callback function
    send_call_back: CallBack,
}

fn empty_func() {}

impl Engine {
    /// create a fuzz engine
    pub fn new(
        message_pool: MessagePool,
        connnected_nodes: Vec<Neighbour>,
        state_model: StateModel,
        cur_state: State,
        sign_call_back: CallBack,
        send_call_back: CallBack,
    ) -> Self {
        Self {
            message_pool,
            connnected_nodes,
            state_model,
            cur_state,
            sign_call_back,
            send_call_back,
        }
    }

    /// create a fuzz engine without any field
    pub fn new_with_empty_fields() -> Self {
        let message_pool = MessagePool::new(Vec::new());
        let connnected_nodes = Vec::new();
        let state_model = StateModel::new(Vec::new());
        let cur_state = State::new(0, false);
        let sign_call_back: CallBack = empty_func;
        let send_call_back: CallBack = empty_func;
        Self {
            message_pool,
            connnected_nodes,
            state_model,
            cur_state,
            sign_call_back,
            send_call_back,
        }
    }

    /// passive sending fuzz packets without initial seeds
    /// this function returns the next message that the fuzzer need to send with LokiMessage type
    /// it will first update the neighbours and statemodels according to the received message
    pub fn passive_sending(&mut self, _rec_message: LokiMessage) -> Result<Option<LokiMessage>> {
        todo!()
        // return Ok(Some(LokiMessage)) or Ok(None) or Err()
    }

    /// passive sending fuzz packets with initial seeds
    pub fn passive_sending_with_seeds(
        &mut self,
        _initial_seeds: Vec<LokiMessage>,
        _rec_message: LokiMessage,
    ) -> Result<Option<LokiMessage>> {
        todo!()
    }

    /// active sending fuzz packets
    pub fn active_sending(&mut self) -> Result<bool> {
        // (self.send_call_back)();
        todo!()
    }

    /// set the addresses of each node
    pub fn set_node_addr(&mut self, _nodes: Vec<Neighbour>) -> Result<bool> {
        self.connnected_nodes = _nodes;
        Ok(true)
    }

    /// get the connnected nodes
    pub fn get_connected_nodes(&self) -> Result<Vec<Neighbour>> {
        Ok(self.connnected_nodes.clone())
    }

    /// get the mutable connected nodes
    pub fn get_mut_connected_nodes(&mut self) -> Result<&mut Vec<Neighbour>> {
        Ok(&mut self.connnected_nodes)
    }

    /// update current state
    pub fn update_cur_state(&mut self) -> Result<bool> {
        todo!()
    }

    /// get current state
    pub fn get_cur_state(&self) -> Result<State> {
        Ok(self.cur_state.clone())
    }

    /// get mut current state
    pub fn get_cur_mut_state(&mut self) -> Result<&mut State> {
        Ok(&mut self.cur_state)
    }

    /// start the fuzz engine, the main controller of the LOKI
    /// First, the engine resisters the callback functions
    /// Then, the engine inits the scepter parser
    /// After that, the engine constructs the state model
    /// Then, the engine start the active sending thread and start fuzz
    pub fn start_fuzz_engine(&mut self) -> Result<()> {
        todo!()
    }

    // some protected functions that can only be called in the engine object
    /// set the packet sending callback function
    #[allow(dead_code)]
    pub fn set_send_callback(&mut self, f: CallBack) -> Result<bool> {
        self.send_call_back = f;
        Ok(true)
    }

    /// set the signing callback function
    #[allow(dead_code)]
    pub fn set_sign_callback(&mut self, f: CallBack) -> Result<bool> {
        self.sign_call_back = f;
        Ok(true)
    }

    /// set the message_pool
    #[allow(dead_code)]
    pub fn set_message_pool(&mut self, new_pool: MessagePool) -> Result<bool> {
        self.message_pool = new_pool;
        Ok(true)
    }

    /// get the message_pool
    #[allow(dead_code)]
    pub fn get_message_pool(&self) -> Result<MessagePool> {
        Ok(self.message_pool.clone())
    }

    /// get the mutable message_pool
    #[allow(dead_code)]
    pub fn get_mut_message_pool(&mut self) -> Result<&mut MessagePool> {
        Ok(&mut self.message_pool)
    }

    /// set the state model
    #[allow(dead_code)]
    pub fn set_state_model(&mut self, new_model: StateModel) -> Result<bool> {
        self.state_model = new_model;
        Ok(true)
    }

    /// get the state model
    #[allow(dead_code)]
    pub fn get_state_model(&self) -> Result<StateModel> {
        Ok(self.state_model.clone())
    }

    /// get the mutable state model
    #[allow(dead_code)]
    pub fn get_mut_state_model(&mut self) -> Result<&mut StateModel> {
        Ok(&mut self.state_model)
    }

    /// get the mutable neighbour node by id
    #[allow(dead_code)]
    pub fn get_mut_neighbour_by_id(&mut self, _node_id: String) -> Result<&mut Neighbour> {
        todo!()
    }

    /// get the neighbour node by id
    #[allow(dead_code)]
    fn get_neighbour_by_id(&self, _node_id: String) -> Result<Neighbour> {
        todo!()
    }
}
