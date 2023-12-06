use alloc::{string::String, collections::BTreeMap};
use crate::{Behaviour, BehaviourRunContext, NodeData, VariableSet, NodeArg, ErrorCode};

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

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        //TODO: Implement this
        return Ok(());
    }
}
