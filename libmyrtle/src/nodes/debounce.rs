use alloc::{boxed::Box, collections::BTreeMap, string::String};

use crate::{
    Behaviour, BehaviourRunContext, ErrorCode, MemoryDataSource, NodeData, NodeArg, Symbol, VariableSet,
};

pub struct DebounceBehaviour {
    last_tick : u64,
    period: i32
}

impl DebounceBehaviour {
    pub fn new() -> DebounceBehaviour {
        DebounceBehaviour {
            last_tick: 0,
            period: 30
        }
    }
}

impl Behaviour for DebounceBehaviour {
    fn is_working(&self) -> bool {
        return false;
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        let in_buf_val = context.in_buf.pop();

        if (context.current_ticks - self.last_tick) > self.period as u64 {
            context.out_buf.push(in_buf_val);
            self.last_tick = context.current_ticks;
        }
    }

    fn reset(&mut self) -> () {
        self.last_tick = 0;
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("period") {
            Some(NodeArg::Base(NodeData::Int(p))) => self.period = p,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}
