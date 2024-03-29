use crate::VariableSet;
use crate::seq::ChainSeq;
use crate::{seq::Seq, Behaviour, BehaviourRunContext, ErrorCode, NodeArg, NodeData};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};

pub struct StreamBehaviour {
    is_streaming: bool,
    seq: Box<dyn Seq>,
}

impl StreamBehaviour {
    pub fn new(emit_seq: Box<dyn Seq>) -> StreamBehaviour {
        StreamBehaviour {
            is_streaming: false,
            seq: Box::new(ChainSeq::new(vec![])),
        }
    }
}

impl Behaviour for StreamBehaviour {
    fn is_working(&self) -> bool {
        self.is_streaming
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        if self.is_streaming {
            match self.seq.poll() {
                Some(s) => context.out_buf.push(s),
                None => {}
            };

            if self.seq.is_done() {
                self.is_streaming = false;
            }
        } else {
            /* Pop the in buffer */
            match context.in_buf.pop() {
                NodeData::Nil => {}
                _ => {
                    self.seq.reset();
                    self.is_streaming = true
                }
            }
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
