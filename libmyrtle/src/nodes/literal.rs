/* Literal */

use alloc::{string::String, collections::BTreeMap};
use crate::{Behaviour, BehaviourRunContext, NodeData, VariableSet, NodeArg, ErrorCode};

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

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("value") {
            Some(NodeArg::Base(value)) => self.value = value,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}