use crate::{var_store::VarStore, node::{Node, NodeData}};

pub struct State {
    flows : Vec<Node>,
    cursor : usize
}

impl State {
    pub fn step(&mut self, vars : &mut VarStore) {
        let cur_flow = self.flows.get_mut(self.cursor).unwrap(); 
        
        if cur_flow.can_receive_data() {
            cur_flow.set_buffer(NodeData::Pulse)
        }

        cur_flow.step(vars);

        self.cursor = (self.cursor + 1) % self.flows.len();
    }

    pub fn are_events_idle(&self) -> bool {
        return self.flows
            .iter()
            .all(|f| f.can_receive_data());
    }

    pub fn new(flows: Vec<Node>) -> State {
        State {
            cursor: 0,
            flows
        }
    }

    pub fn reset(&mut self){
        self.cursor = 0;
        self.flows.iter_mut().for_each(|f| f.reset())
    }
}