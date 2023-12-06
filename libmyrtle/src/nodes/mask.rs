use crate::VariableSet;
use crate::seq::ChainSeq;
use crate::{
    seq::{RepeatSeq, Seq},
    Behaviour, BehaviourRunContext, ErrorCode, NodeArg, NodeData,
};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};

pub struct MaskBehaviour {
    seq: Box<dyn Seq>,
}

impl MaskBehaviour {
    pub fn new() -> MaskBehaviour {
        MaskBehaviour {
            seq: Box::new(ChainSeq::new(vec![])),
        }
    }
}

impl Behaviour for MaskBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        /* Pop the in buffer */
        let data = context.in_buf.pop();

        match self.seq.push(data) {
            Some(s) if s == NodeData::Nil => {}
            Some(s) => context.out_buf.push(s),
            None => {
                context.out_buf.push(NodeData::End);
                self.seq.reset();
            }
        };

        //TODO: If successful, send a pulse to a callback symbol
        if self.seq.is_done() {
            self.seq.reset();
        }
    }

    fn reset(&mut self) -> () {
        self.seq.reset();
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("mask") {
            Some(NodeArg::Seq(seq)) => self.seq = seq,
            None => Err(ErrorCode::ArgumentRequired)?,
            _ => Err(ErrorCode::InvalidArgumentType)?,
        };

        Ok(())
    }
}
