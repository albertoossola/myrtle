use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use alloc::boxed::Box;
use core::fmt::Write;

use crate::{
    node::{Node, NodeRunContext},
    nodedata::NodeData,
    symbols::Symbol,
};

pub struct MachineRunContext {
    pub current_ticks: u64,
    pub current_ticks_us: u64,
}

pub struct StateRunContext<'a> {
    pub machine_vars: &'a mut BTreeMap<String, Symbol>,
    pub current_ticks: u64,
    pub current_ticks_us: u64,
    pub current_state: &'a mut String
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
                current_ticks_us: context.current_ticks_us,
                current_state: context.current_state
            };

            s.run(flow_context);
        }
    }

    pub fn reset(&mut self){
        for flow in self.flows.iter_mut() {
            flow.reset();
        }
    }
}

/* Machine */
pub struct Machine {
    pub states: BTreeMap<String, State>,
    pub variables: BTreeMap<String, Symbol>,
    pub cur_state: String
}

impl Machine {
    pub fn run(&mut self, context: MachineRunContext) {
        let mut next_state: String = String::from(&self.cur_state);

        {
            let state: &mut State = self.states.get_mut(&self.cur_state).unwrap();

            state.run(StateRunContext {
                current_state: &mut next_state,
                machine_vars: &mut self.variables,
                current_ticks: context.current_ticks,
                current_ticks_us: context.current_ticks_us,
            });

            if next_state != self.cur_state {
                state.reset();
            }
        }

        if !self.states.contains_key(&next_state) {
            next_state.clear();
            next_state.write_str("entry").ok();
        }

        self.cur_state = next_state;
    }

    pub fn make_blank() -> Machine {
        let mut states = BTreeMap::new();
        states.insert(
            "entry".to_string(),
            State {
                vars: BTreeMap::new(),
                flows: alloc::vec![],
            },
        );

        Machine {
            cur_state: String::from("entry"),
            states,
            variables: BTreeMap::new()
        }
    }
}
