/* Buffer */

use crate::{nodedata::NodeData, symbols::Symbol};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use core::slice::Iter;

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
}

/* RunContext */

pub struct BehaviourRunContext<'a> {
    pub in_buf: &'a mut NodeBuffer,
    pub out_buf: &'a mut NodeBuffer,
    pub machine_vars: &'a mut BTreeMap<String, Symbol>,
    pub state_vars: &'a mut BTreeMap<String, Symbol>,
    pub current_ticks: u64,
}

/* Behaviour */

pub trait Behaviour {
    fn is_working(&self) -> bool;
    fn run(&mut self, context: BehaviourRunContext) -> ();
    fn reset(&mut self) -> ();
}

/* Node */

pub struct NodeRunContext<'a> {
    pub machine_vars: &'a mut BTreeMap<String, Symbol>,
    pub state_vars: &'a mut BTreeMap<String, Symbol>,
    pub current_ticks: u64,
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
            };

            self.behaviour.run(behaviour_context);
        } else {
            match self.next.as_mut() {
                Some(next) => next.run(context),
                None => {}
            }
        }
    }
}

/* --- Behaviours --- */

/* SetVar */
pub struct SetVarBehaviour {
    var_name: String,
    value_to_set: NodeData,
}

impl SetVarBehaviour {
    pub fn new(var_name: String) -> SetVarBehaviour {
        SetVarBehaviour {
            var_name,
            value_to_set: NodeData::Nil,
        }
    }
}

impl Behaviour for SetVarBehaviour {
    fn is_working(&self) -> bool {
        match self.value_to_set {
            NodeData::Nil => false,
            _ => true,
        }
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        if !self.is_working() {
            self.value_to_set = context.in_buf.pop();
        }

        let var = context
            .machine_vars
            .get_mut(self.var_name.as_str())
            .unwrap();

        if var.can_push() {
            var.push(self.value_to_set);
            self.value_to_set = NodeData::Nil;
        }
    }

    fn reset(&mut self) -> () {
        todo!()
    }
}

/* Emit */

pub struct EmitBehaviour {
    values: Vec<NodeData>,
    cur: usize,
}

impl EmitBehaviour {
    pub fn new(values: Vec<NodeData>) -> EmitBehaviour {
        EmitBehaviour { values, cur: 0 }
    }
}

impl Behaviour for EmitBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        /* Pop the in buffer */
        context.in_buf.pop();

        let current_value = self.values[self.cur];

        context.out_buf.push(current_value);

        self.cur += 1;
        if self.cur >= self.values.len() {
            self.cur = 0;
        }
    }

    fn reset(&mut self) -> () {
        self.cur = 0;
    }
}

/* Literal */

pub struct LiteralBehaviour {
    value: NodeData,
}

impl LiteralBehaviour {
    pub fn new(data: NodeData) -> LiteralBehaviour {
        LiteralBehaviour { value: data }
    }
}

impl Behaviour for LiteralBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        context.out_buf.push(self.value);
    }

    fn reset(&mut self) -> () {}
}

/* PrintBehaviour */
pub struct PrintBehaviour {}

impl Behaviour for PrintBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        let data = context.in_buf.pop();
    }

    fn reset(&mut self) -> () {}
}

/* TimerBehaviour */

pub struct TimerBehaviour {
    pub interval: u64,
    pub last_tick: u64,
}

impl TimerBehaviour {
    pub fn new(interval: u64) -> TimerBehaviour {
        TimerBehaviour {
            interval: interval,
            last_tick: 0,
        }
    }
}

impl Behaviour for TimerBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        let elapsed = context.current_ticks - self.last_tick;
        if elapsed > self.interval {
            self.last_tick = context.current_ticks;

            context.out_buf.push(NodeData::Int(1));
        }
    }

    fn reset(&mut self) -> () {}
}

/* CountBehaviour */
pub struct CountBehaviour {
    pub c: i32,
}

impl Behaviour for CountBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        context.in_buf.pop();
        self.c += 1;
        context.out_buf.push(NodeData::Int(self.c));
    }

    fn reset(&mut self) -> () {
        self.c = 0;
    }
}
