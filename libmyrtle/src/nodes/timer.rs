use alloc::{string::String, collections::BTreeMap, boxed::Box};

use crate::{Behaviour, BehaviourRunContext, NodeData, VariableSet, NodeArg, ErrorCode, MemoryDataSource, Symbol};

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

    fn reset(&mut self) -> () {
        self.last_tick = 0;
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("ms") {
            Some(NodeArg::Base(NodeData::Int(var))) => self.ms = var as u64,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}