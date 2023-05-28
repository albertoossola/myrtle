use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::{Behaviour, BehaviourRunContext, ErrorCode, NodeData, NodeArg};

pub struct EqualsBehaviour {
    pub value : NodeData
}

impl EqualsBehaviour {
    pub fn new() -> EqualsBehaviour {
        EqualsBehaviour {
            value: NodeData::Int(0)
        }
    }
}

impl Behaviour for EqualsBehaviour {
    fn is_working(&self) -> bool {  false  }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        match context.in_buf.pop() {
            data if data == self.value => context.out_buf.push(data),
            _ => {}
        }
    }

    fn reset(&mut self) -> () { }

    fn init(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("value") {
            Some(NodeArg::Base(data)) => {
                self.value = data;
                return Ok(());
            },
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        }
    }
}