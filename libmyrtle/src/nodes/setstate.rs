use alloc::{boxed::Box, collections::BTreeMap, string::String};
use core::fmt::Write;

use crate::{
    Behaviour, BehaviourRunContext, ErrorCode, MemoryDataSource, NodeData, NodeArg, Symbol,
};

pub struct SetStateBehaviour {
    state: String
}

impl SetStateBehaviour {
    pub fn new(state: String) -> SetStateBehaviour {
        SetStateBehaviour {
            state
        }
    }
}

impl Behaviour for SetStateBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        match context.in_buf.pop(){
            NodeData::Nil => {},
            _ => {
                context.current_state.clear();
                context.current_state.write_str(&self.state).unwrap();
            }
        }
    }

    fn reset(&mut self) -> () {
        todo!()
    }

    fn init(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("state") {
            Some(NodeArg::String(var)) => self.state = var,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}
