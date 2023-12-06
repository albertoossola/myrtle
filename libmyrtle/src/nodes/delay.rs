use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::{Behaviour, BehaviourRunContext, ErrorCode, NodeData, NodeArg, VariableSet};

pub struct DelayBehaviour {
    pub interval : i32,
    pub buffer : NodeData,
    pub received_at : u64
}

impl DelayBehaviour {
    pub fn new() -> DelayBehaviour {
        DelayBehaviour {
            interval: 1000,
            buffer: NodeData::Nil,
            received_at: 0
        }
    }
}

impl Behaviour for DelayBehaviour {
    fn is_working(&self) -> bool {
        self.buffer != NodeData::Nil
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        if self.buffer != NodeData::Nil {
            if (context.current_ticks - self.received_at) > self.interval as u64 {
                context.out_buf.push(self.buffer);
                self.buffer = NodeData::Nil;
            }
        }
        else {
            match context.in_buf.pop() {
                NodeData::Nil => {},
                data => {
                    self.buffer = data;
                    self.received_at = context.current_ticks;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.buffer = NodeData::Nil;
        self.received_at = 0;
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("ms") {
            Some(NodeArg::Base(NodeData::Int(p))) => self.interval = p,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}