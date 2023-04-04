use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{
    node::{Node, NodeRunContext},
    nodedata::NodeData,
    symbols::Symbol,
};

pub struct MachineRunContext {
    pub current_ticks: u64,
}

pub struct StateRunContext<'a> {
    pub machine_vars: &'a mut BTreeMap<String, Symbol>,
    pub current_ticks: u64,
}

pub struct State {
    pub vars: BTreeMap<String, Symbol>,
    pub flows: Vec<Node>,
}

impl State {
    pub fn run(&mut self, context: StateRunContext) {
        for s in self.flows.iter_mut() {
            s.in_buf.push(NodeData::Int(1));

            let flow_context = NodeRunContext {
                state_vars: &mut self.vars,
                machine_vars: context.machine_vars,
                current_ticks: context.current_ticks,
            };

            s.run(flow_context);
        }
    }
}

/* Machine */
pub struct Machine {
    pub states: BTreeMap<String, State>,
    pub variables: BTreeMap<String, Symbol>,
    pub cur_state: String,
}

impl Machine {
    pub fn run(&mut self, context: MachineRunContext) {
        let state: &mut State = self.states.get_mut(&self.cur_state).unwrap();
        state.run(StateRunContext {
            machine_vars: &mut self.variables,
            current_ticks: context.current_ticks,
        });
    }
}
