//! the state model in LOKI

use anyhow::{anyhow, Result};

/// the struct of state model
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StateModel {
    states: Vec<State>,
}

/// the struct of state edges
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StateEdge {
    /// the end state of the edge
    to_state: String,
    /// the weight of this edge
    weight: f32,
}

/// the struct of state
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    /// the name of the state, which is also the message type of current State
    msg_type: String,
    /// the exit edges of current State
    edges: Vec<StateEdge>,
}

impl State {
    /// construct a new state
    pub fn new(msg_type: String) -> Self {
        let edges = Vec::new();
        Self { msg_type, edges }
    }

    /// construct a new empty State
    pub fn new_empty() -> Self {
        let edges = Vec::new();
        Self {
            msg_type: "".to_string(),
            edges,
        }
    }

    /// get the message type of the state
    pub fn get_msg_type(&self) -> String {
        self.msg_type.clone()
    }

    /// get the mutable message type of the state
    pub fn get_mut_msg_type(&mut self) -> &mut String {
        &mut self.msg_type
    }

    /// set the message type of the state
    pub fn set_msg_type(&mut self, new_type: String) -> Result<bool> {
        self.msg_type = new_type;
        Ok(true)
    }

    /// get current edges
    pub fn get_cur_edges(&self) -> &Vec<StateEdge> {
        &self.edges
    }

    /// get mutable current edges
    pub fn get_mut_cur_edges(&mut self) -> &mut Vec<StateEdge> {
        &mut self.edges
    }

    /// set the current edges
    pub fn add_cur_edges(&mut self, new_edge: StateEdge) -> Result<bool> {
        self.edges.push(new_edge);
        Ok(true)
    }

    /// judge whether it is an empty State
    pub fn is_empty(&self) -> Result<bool> {
        if self.msg_type == "".to_string() {
            return Ok(true);
        }
        Ok(false)
    }
}

impl StateEdge {
    /// construct a new state edge
    pub fn new(to_state: String, weight: f32) -> Self {
        Self { to_state, weight }
    }

    /// set the to_state
    pub fn set_to_state(&mut self, new_to: String) -> Result<bool> {
        self.to_state = new_to;
        Ok(true)
    }

    /// get the to_state
    pub fn get_to_state(&self) -> String {
        self.to_state.clone()
    }

    /// get mutable to_state
    pub fn get_mut_to_state(&mut self) -> &mut String {
        &mut self.to_state
    }

    /// set the weight
    pub fn set_weight(&mut self, new_weight: f32) -> Result<bool> {
        self.weight = new_weight;
        Ok(true)
    }

    /// get the weight
    pub fn get_weight(&self) -> f32 {
        self.weight
    }
}

impl StateModel {
    /// construct a new state model
    pub fn new(states: Vec<State>) -> Self {
        Self { states }
    }

    /// add a new state
    pub fn add_new_state(&mut self, new_state: State) -> Result<bool> {
        self.states.push(new_state);
        Ok(true)
    }

    /// delete a state
    pub fn delete_state(&mut self, state_name: String) -> Result<bool> {
        // first find the state and retain the other states
        self.states.retain(|state| {
            let delete = { state.get_msg_type() == state_name };
            !delete
        });
        // if the state does not exist, don't return error ethier
        Ok(true)
    }

    /// find the State according to the message type
    pub fn find_state_with_msg_type(&self, msg_type: String) -> Result<&State> {
        match self.states.iter().find(|s| s.get_msg_type() == msg_type) {
            Some(state) => {
                return Ok(state);
            }
            None => {
                return Err(anyhow!("cannot find the message with type: {:?}", msg_type));
            }
        }
    }

    /// find the mutable State according to the message type
    pub fn find_mut_state_with_msg_type(&mut self, msg_type: String) -> Result<&mut State> {
        match self
            .states
            .iter_mut()
            .find(|s| s.get_msg_type() == msg_type)
        {
            Some(state) => {
                return Ok(state);
            }
            None => {
                return Err(anyhow!("cannot find the message with type: {:?}", msg_type));
            }
        }
    }
}
