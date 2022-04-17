//! LOKI fuzzing engine

use crate::loki_message::LokiMessage;
use crate::message_pool::MessagePool;
use crate::neighbour::Neighbour;
// use crate::error::LOKIError;
use crate::state_model::{State, StateEdge, StateModel};
use anyhow::{anyhow, Context, Result};
use rand::Rng;

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
    cur_state: String,

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
        cur_state: String,
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
        let cur_state = "".to_string();
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
    pub fn passive_sending(&mut self, rec_message: LokiMessage) -> LokiMessage {
        // first, update the neighbours
        match self.update_neighbours_with_rec_msg(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the neighbours with received message!")
            }
        };

        // judge sender's current state, if it needs to transfer, choose the proper state
        match self.update_sender_node_state(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // check whethere the received message leads to a new edge
        match self.check_and_update_new_edges(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // Then, we need to decide which State to transfer, and what type of messages we need to send
        let next_msg_type = match self.get_next_message_type_and_update_state() {
            Ok(msg_type) => msg_type,
            Err(_) => {
                panic!("Failed to get next message type")
            }
        };

        // add the received message into the message pool
        self.get_mut_message_pool()
            .unwrap()
            .add_msg_to_pool(rec_message)
            .unwrap();

        // get the latest message of the next message type from the message pool
        let mut org_msg = match self
            .get_message_pool()
            .unwrap()
            .find_latest_message_with_type(next_msg_type.clone())
        {
            Ok(msg) => msg,
            Err(_) => {
                // no fit messages found in the message pool, generate a new message
                LokiMessage::generate(next_msg_type.clone())
            }
        };

        // mutate the message and return
        let next_msg = org_msg.mutate();
        next_msg
    }

    /// passive sending fuzz packets with initial seeds
    pub fn passive_sending_with_seed(
        &mut self,
        based_seed: &mut LokiMessage,
        rec_message: LokiMessage,
    ) -> LokiMessage {
        // first, update the neighbours
        match self.update_neighbours_with_rec_msg(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the neighbours with received message!")
            }
        };

        // judge sender's current state, if it needs to transfer, choose the proper state
        match self.update_sender_node_state(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // check whethere the received message leads to a new edge
        match self.check_and_update_new_edges(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // add the received message into the message pool
        self.get_mut_message_pool()
            .unwrap()
            .add_msg_to_pool(rec_message)
            .unwrap();

        // update the current state of loki
        let next_msg_type = based_seed.get_msg_type();

        match self
            .get_mut_state_model()
            .find_state_with_msg_type(next_msg_type.clone())
        {
            Ok(_) => {
                self.set_cur_state(next_msg_type).unwrap();
            }
            Err(_) => {
                // This indicates that a new state need to be added into the state model
                let new_state = State::new(next_msg_type.clone());
                self.get_mut_state_model().add_new_state(new_state).unwrap();
                self.set_cur_state(next_msg_type).unwrap();
            }
        };

        // mutate the message and return
        let next_msg = based_seed.mutate();
        next_msg
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
    pub fn set_cur_state(&mut self, new_state: String) -> Result<bool> {
        self.cur_state = new_state;
        Ok(true)
    }

    /// get current state
    pub fn get_cur_state(&self) -> String {
        self.cur_state.clone()
    }

    /// get mut current state
    pub fn get_cur_mut_state(&mut self) -> &mut String {
        &mut self.cur_state
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
    pub fn get_state_model(&self) -> &StateModel {
        &self.state_model
    }

    /// get the mutable state model
    pub fn get_mut_state_model(&mut self) -> &mut StateModel {
        &mut self.state_model
    }

    /// get the mutable neighbour node by id
    pub fn get_mut_neighbour_by_id(&mut self, node_id: String) -> Result<&mut Neighbour> {
        for n in &mut self.connnected_nodes {
            if n.get_node_id() == node_id {
                return Ok(n);
            }
        }
        return Err(anyhow!(
            "Try to get a neighbour node not connected with LOKI!"
        ));
    }

    /// get the neighbour node by id
    pub fn get_neighbour_by_id(&self, node_id: String) -> Result<&Neighbour> {
        for n in &self.connnected_nodes {
            if n.get_node_id() == node_id {
                return Ok(n);
            }
        }
        return Err(anyhow!(
            "Try to get a neighbour node not connected with LOKI!"
        ));
    }

    /// update the connected nodes according to the received messages
    pub fn update_neighbours_with_rec_msg(&mut self, rec_message: &LokiMessage) -> Result<bool> {
        let from_node = match rec_message.get_from_node() {
            Ok(node) => node,
            Err(e) => {
                return Err(e).with_context(|| format!("Received a Message without From Node"));
            }
        };
        // judge whether the from node is included in the connected_nodes
        let node_ids: Vec<_> = self
            .connnected_nodes
            .iter()
            .map(|n| n.get_node_id())
            .collect();
        // if not, add the current node into the neighbours
        if !node_ids.contains(&from_node) {
            self.connnected_nodes.push(Neighbour::new(
                from_node.clone(),
                rec_message.get_msg_type(),
            ))
        }
        Ok(true)
    }

    /// update the sender nodes state
    pub fn update_sender_node_state(&mut self, rec_message: &LokiMessage) -> Result<bool> {
        let from_node = match rec_message.get_from_node() {
            Ok(node) => node,
            Err(e) => {
                return Err(e).with_context(|| format!("Received a Message without From Node"));
            }
        };
        let new_type = rec_message.get_msg_type();

        // After that, update the sender's state, first we find the sender node
        let sender_node: &mut Neighbour = self
            .connnected_nodes
            .iter_mut()
            .find(|n| n.get_node_id() == from_node)
            .unwrap();

        // update the sender's state
        sender_node.set_current_state(new_type.clone()).unwrap();
        return Ok(true);
    }

    /// check whether we should add new edges to the state model when received some msgs
    pub fn check_and_update_new_edges(&mut self, rec_message: &LokiMessage) -> Result<bool> {
        let new_type = rec_message.get_msg_type();
        let cur_type = self.get_cur_state();
        // update the edges if there is no edge before, the initial weight is 1.0
        let cur_state = match self
            .get_mut_state_model()
            .find_mut_state_with_msg_type(cur_type)
        {
            Ok(state) => state,
            Err(e) => return Err(e),
        };
        let e = cur_state
            .get_cur_edges()
            .iter()
            .find(|e| e.get_to_state() == new_type);
        if e.is_none() {
            let new_edge = StateEdge::new(new_type.clone(), 1.0);
            cur_state.add_cur_edges(new_edge).unwrap();
        }
        Ok(true)
    }

    /// get the next message type according to the transfer weight
    pub fn get_next_message_type_and_update_state(&mut self) -> Result<String> {
        let cur_state_type = self.get_cur_state();
        let cur_state = match self
            .get_state_model()
            .find_state_with_msg_type(cur_state_type)
        {
            Ok(state) => state,
            Err(e) => return Err(e),
        };
        let weigths: Vec<f32> = cur_state
            .get_cur_edges()
            .iter()
            .map(|e| e.get_weight())
            .collect();
        // get the total weights
        let total_weights: f32 = weigths.iter().sum();
        let mut to_state = String::new();
        let mut rng = rand::thread_rng();
        let mut min_range: u32 = 0;
        let mut max_range: u32 = 0;
        let temp: u32 = rng.gen_range(0..101);
        for edge in cur_state.get_cur_edges().iter() {
            max_range += ((edge.get_weight() / total_weights) as u32) * 100;
            if min_range <= temp && max_range >= temp {
                to_state = edge.get_to_state();
                break;
            }
            min_range = max_range;
        }
        // if the to state is empty, something is wrong
        if to_state.is_empty() {
            return Err(anyhow!(
                "can't find any state to transfer, some states may have no edges in the statemodel"
            ));
        } else {
            self.set_cur_state(to_state).unwrap();
        }
        // get the type of next message
        Ok(self.get_cur_state())
    }
}
