use crate::{
    seq::{RepeatSeq, Seq},
    Behaviour, BehaviourRunContext, ErrorCode, NodeArg, VariableSet,
};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};
use crate::seq::ChainSeq;

pub struct EmitBehaviour {
    seq: Box<dyn Seq>,
}

impl EmitBehaviour {
    pub fn new(emit_seq: Box<dyn Seq>) -> EmitBehaviour {
        EmitBehaviour {
            seq: Box::new(ChainSeq::new(vec![]))
        }
    }
}

impl Behaviour for EmitBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        /* Pop the in buffer */
        context.in_buf.pop();

        match self.seq.poll() {
            Some(s) => context.out_buf.push(s),
            None => {}
        };

        if self.seq.is_done() {
            self.seq.reset();
        }
    }

    fn reset(&mut self) -> () {
        self.seq.reset();
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("items") {
            Some(NodeArg::Seq(seq)) => self.seq = seq,
            None => Err(ErrorCode::ArgumentRequired)?,
            _ => Err(ErrorCode::InvalidArgumentType)?,
        };

        Ok(())
    }
}
