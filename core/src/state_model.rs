//! the state model in LOKI

use anyhow::Result;

/// the struct of state model
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StateModel {
    edges: Vec<StateEdge>,
}

/// the struct of state edges
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StateEdge {
    /// the end state of the edge
    to_state: State,
    /// the weight of this edge
    weight: f32,
}

/// the struct of state
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State {
    /// the state id
    id: u32,
    /// the name of the state, which is also the message type of current State
    msg_type: String,
    /// is the state the current one
    cur_state: bool,
    /// the count of certain type of the message
    count: u32,
    /// the exit edges of current State
    edges: Vec<StateEdge>,
}

impl State {
    /// construct a new state
    pub fn new(id: u32, msg_type: String, cur_state: bool, count: u32) -> Self {
        let edges = Vec::new();
        Self {
            id,
            msg_type,
            cur_state,
            count,
            edges,
        }
    }

    /// construct a new empty State
    pub fn new_empty() -> Self {
        let edges = Vec::new();
        Self {
            id: 0,
            msg_type: "".to_string(),
            cur_state: false,
            count: 0,
            edges,
        }
    }

    /// set the cur_state value
    pub fn set_cur_state(&mut self, cur_state: bool) -> Result<bool> {
        self.cur_state = cur_state;
        Ok(true)
    }

    /// get the cur_state value
    pub fn get_cur_state(&self) -> Result<bool> {
        Ok(self.cur_state)
    }

    /// set the state id
    pub fn set_state_id(&mut self, new_id: u32) -> Result<bool> {
        self.id = new_id;
        Ok(true)
    }

    /// get the state id
    pub fn get_state_id(&self) -> Result<u32> {
        Ok(self.id)
    }

    /// get current edges
    pub fn get_cur_edges(&self) -> Result<Vec<StateEdge>> {
        Ok(self.edges.clone())
    }

    /// get mutable current edges
    pub fn get_mut_cur_edges(&mut self) -> Result<&mut Vec<StateEdge>> {
        Ok(&mut self.edges)
    }

    /// set the current edges
    pub fn add_cur_edges(&mut self, new_edge: StateEdge) -> Result<bool> {
        self.edges.push(new_edge);
        Ok(true)
    }
}

impl StateEdge {
    /// construct a new state edge
    pub fn new(to_state: State, weight: f32) -> Self {
        Self { to_state, weight }
    }

    /// set the to_state
    pub fn set_to_state(&mut self, new_to: State) -> Result<bool> {
        self.to_state = new_to;
        Ok(true)
    }

    /// get the to_state
    pub fn get_to_state(&self) -> Result<State> {
        Ok(self.to_state.clone())
    }

    /// get mutable to_state
    pub fn get_mut_to_state(&mut self) -> Result<&mut State> {
        Ok(&mut self.to_state)
    }

    /// set the weight
    pub fn set_weight(&mut self, new_weight: f32) -> Result<bool> {
        self.weight = new_weight;
        Ok(true)
    }

    /// get the weight
    pub fn get_weight(&self) -> Result<f32> {
        Ok(self.weight)
    }
}

impl StateModel {
    /// construct a new state model
    pub fn new(edges: Vec<StateEdge>) -> Self {
        Self { edges }
    }

    /// add new edges
    pub fn add_new_edge(&mut self, _new_edge: StateEdge) -> Result<bool> {
        // check whether the edge exists and then add it
        todo!()
    }

    /// delete some edges
    pub fn delete_edge(&mut self, _some_edge: StateEdge) -> Result<bool> {
        // check whether the edge exists and then delete it
        todo!()
    }
}
