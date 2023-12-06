/* Buffer */

use crate::{nodedata::NodeData, symbols::Symbol, ErrorCode, MemoryDataSource, NodeArg, VariableSet};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

pub struct NodeBuffer {
    pub data: NodeData,
}

impl NodeBuffer {
    pub fn is_full(&self) -> bool {
        match self.data {
            NodeData::Nil => false,
            _ => true,
        }
    }

    pub fn push(&mut self, data: NodeData) -> () {
        match self.data {
            NodeData::Nil => {
                self.data = data;
            }
            _ => {}
        };
    }

    pub fn pop(&mut self) -> NodeData {
        let cur_data = self.data;
        self.data = NodeData::Nil;
        return cur_data;
    }

    pub fn peek(&self) -> NodeData {
        self.data
    }
}

/* RunContext */

pub struct BehaviourRunContext<'a> {
    pub in_buf: &'a mut NodeBuffer,
    pub out_buf: &'a mut NodeBuffer,
    pub machine_vars: &'a mut BTreeMap<String, Symbol>,
    pub state_vars: &'a mut BTreeMap<String, Symbol>,
    pub current_ticks: u64,
    pub current_ticks_us: u64,
    pub current_state: &'a mut String
}

/* Behaviour */

pub trait Behaviour {
    fn is_working(&self) -> bool { return false; }
    fn run(&mut self, context: BehaviourRunContext) -> ();
    fn reset(&mut self) -> ();
    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode>;
    fn init(&mut self, vars: &mut VariableSet) -> () { }
    fn on_state_change(&mut self, vars: &mut VariableSet) -> () { }
    fn on_state_enter(&mut self, vars: &mut VariableSet) -> () { }
}

/* Node */

pub struct NodeRunContext<'a> {
    pub machine_vars: &'a mut BTreeMap<String, Symbol>,
    pub state_vars: &'a mut BTreeMap<String, Symbol>,
    pub current_ticks: u64,
    pub current_ticks_us: u64,
    pub current_state: &'a mut String
}

pub struct Node {
    pub in_buf: NodeBuffer,
    pub behaviour: Box<dyn Behaviour>,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn run(&mut self, context: NodeRunContext) -> () {
        let next_free = match &self.next {
            Some(next) => !next.in_buf.is_full(),
            None => true,
        };

        let mut sink = NodeBuffer {
            data: NodeData::Nil,
        };

        let out_buf = match self.next.as_mut() {
            Some(next) => &mut next.in_buf,
            None => &mut sink,
        };

        let has_input = self.in_buf.is_full();

        if next_free && (has_input || self.behaviour.is_working()) {
            let behaviour_context = BehaviourRunContext {
                in_buf: &mut self.in_buf,
                out_buf: out_buf,
                machine_vars: context.machine_vars,
                state_vars: context.state_vars,
                current_ticks: context.current_ticks,
                current_ticks_us: context.current_ticks_us,
                current_state: context.current_state
            };

            self.behaviour.run(behaviour_context);
        }
        match self.next.as_mut() {
            Some(next) => next.run(context),
            None => {}
        }
    }

    pub fn init(&mut self, vars: &mut VariableSet) {
        self.behaviour.init(vars);

        match self.next.as_mut() {
            Some(next) => next.init(vars),
            None => {}
        }
    }

    pub fn reset(&mut self) {
        self.behaviour.reset();
        self.in_buf.pop();
    }

    pub fn on_state_enter(&mut self, vars: &mut VariableSet) {
        self.behaviour.on_state_enter(vars);

        match self.next.as_mut() {
            Some(next) => next.on_state_enter(vars),
            None => {}
        }
    }

    pub fn on_state_change(&mut self, vars: &mut VariableSet) {
        self.behaviour.on_state_change(vars);

        match self.next.as_mut() {
            Some(next) => next.on_state_change(vars),
            None => {}
        }
    }
}