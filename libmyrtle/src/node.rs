/* Buffer */

use crate::{nodedata::NodeData, symbols::Symbol, ErrorCode, MemoryDataSource, NodeParam};
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
}

/* Behaviour */

pub trait Behaviour {
    fn is_working(&self) -> bool;
    fn run(&mut self, context: BehaviourRunContext) -> ();
    fn reset(&mut self) -> ();
    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode>;
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

/* WatchVar */
pub struct WatchVarBehaviour {
    var_name: String,
    listener_id: i32,
}

impl WatchVarBehaviour {
    pub fn new() -> WatchVarBehaviour {
        WatchVarBehaviour {
            var_name: String::from(""),
            listener_id: -1,
        }
    }
}

impl Behaviour for WatchVarBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        if !context.machine_vars.contains_key(&self.var_name) {
            context.machine_vars.insert(
                self.var_name.clone(),
                Symbol::new(Box::new(MemoryDataSource::new())),
            );
        }

        let var = context.machine_vars.get_mut(&self.var_name).unwrap();

        if self.listener_id == -1 {
            self.listener_id = var.register_listener();
            var.activate_listener(self.listener_id);
        }

        let polled = var.poll(self.listener_id);

        match polled {
            NodeData::Nil => {}
            _ => {
                context.out_buf.push(polled);
            }
        };
    }

    fn reset(&mut self) -> () {}

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        self.listener_id = -1;

        match args.remove("var") {
            Some(NodeParam::String(var)) => self.var_name = var,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}

/* Literal */

pub struct LiteralBehaviour {
    value: NodeData,
}

impl LiteralBehaviour {
    pub fn new() -> LiteralBehaviour {
        LiteralBehaviour {
            value: NodeData::Nil,
        }
    }
}

impl Behaviour for LiteralBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        match context.in_buf.pop() {
            NodeData::Nil => {}
            _ => context.out_buf.push(self.value),
        };
    }

    fn reset(&mut self) -> () {}

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        match args.remove("value") {
            Some(NodeParam::Base(value)) => self.value = value,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
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

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        //TODO: Implement this
        return Ok(());
    }
}

/* TimerBehaviour */

pub struct TimerBehaviour {
    pub ms: u64,
    pub last_tick: u64,
}

impl TimerBehaviour {
    pub fn new(interval: u64) -> TimerBehaviour {
        TimerBehaviour {
            ms: interval,
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
        if elapsed > self.ms {
            self.last_tick = context.current_ticks;

            context.out_buf.push(NodeData::Int(1));
        }
    }

    fn reset(&mut self) -> () {}

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        match args.remove("ms") {
            Some(NodeParam::Base(NodeData::Int(var))) => self.ms = var as u64,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
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

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        //TODO: Implement this
        return Ok(());
    }
}
