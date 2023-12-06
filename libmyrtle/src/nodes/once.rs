use alloc::{boxed::Box, collections::BTreeMap, string::String};

use crate::{
    Behaviour, BehaviourRunContext, ErrorCode, MemoryDataSource, NodeData, NodeArg, Symbol, VariableSet,
};

pub struct OnceBehaviour {
    ran: bool
}

impl OnceBehaviour {
    pub fn new() -> OnceBehaviour {
        OnceBehaviour {
            ran: false
        }
    }
}

impl Behaviour for OnceBehaviour {
    fn is_working(&self) -> bool {
        return false;
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        if self.ran {
            return;
        }

        self.ran = true;
        context.out_buf.push(NodeData::Int(1));
    }

    fn on_state_enter(&mut self, vars: &mut VariableSet) -> () {
        self.ran = false;   
    }

    fn reset(&mut self) -> () {
        self.ran = false;
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        Ok(())
    }
}