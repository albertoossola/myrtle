use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    node::{Node, NodeRunContext},
    nodedata::NodeData,
    symbols::Symbol, ErrorCode,
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
    pub suspended: bool
}

impl State {
    pub fn run(&mut self, context: StateRunContext) {
        if self.suspended {
            return;
        }

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

    pub fn init(&mut self, vars: &mut VariableSet) {
        for flow in self.flows.iter_mut() {
            flow.init(vars);
        }
    }

    pub fn suspend(&mut self, vars : &mut VariableSet) {
        self.flows.iter_mut().for_each(|flow| flow.behaviour.on_state_change(vars));
        self.suspended = true;
    }

    pub fn resume(&mut self, vars : &mut VariableSet) {
        for flow in self.flows.iter_mut() {
            flow.on_state_enter(vars);
        }

        self.flows.iter_mut().for_each(|flow| flow.behaviour.on_state_enter(vars));
        self.suspended = false;
    }

    pub fn reset(&mut self){
        for flow in self.flows.iter_mut() {
            flow.reset();
        }
    }
}

pub type VariableSet = BTreeMap<String, Symbol>;

/* Machine */
pub struct Machine {
    pub states: BTreeMap<String, State>,
    pub variables: VariableSet,
    pub cur_state: String
}

impl Machine {
    fn set_state(&mut self, state_name: &str) {
        if !self.states.contains_key(state_name) {
            return;
        }

        let current_state = self.states.get_mut(&self.cur_state).unwrap();
        current_state.suspend(&mut self.variables);
        
        let new_state = self.states.get_mut(state_name).unwrap();
        new_state.resume(&mut self.variables);

        self.cur_state = String::from(state_name);
    }

    pub fn init(&mut self) -> Result<(), ErrorCode> {        
        self.states.iter_mut().for_each(|(_, state)| state.init(&mut self.variables));
        
        self.cur_state = String::from("entry");
        self.states.get_mut(self.cur_state.as_str())
            .ok_or(ErrorCode::EntryStateRequired)?
            .resume(&mut self.variables);

        Ok(())
    }

    pub fn run(&mut self, context: MachineRunContext) {
        let mut next_state_name: String = String::from(&self.cur_state);

        {
            let current_state: &mut State = self.states.get_mut(&self.cur_state).unwrap();

            current_state.run(StateRunContext {
                current_state: &mut next_state_name,
                machine_vars: &mut self.variables,
                current_ticks: context.current_ticks,
                current_ticks_us: context.current_ticks_us,
            });

            if next_state_name != self.cur_state {
                self.set_state(&next_state_name);
            }
        }
    }

    pub fn make_blank() -> Machine {
        let mut states = BTreeMap::new();

        let entry_state = State {
            vars: BTreeMap::new(),
            flows: alloc::vec![],
            suspended: true
        };

        states.insert("entry".to_string(), entry_state);

        Machine {
            cur_state: String::from("entry"),
            states,
            variables: BTreeMap::new()
        }
    }
}
