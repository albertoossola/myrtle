use crate::{seq::{RepeatSeq, Seq}, Behaviour, BehaviourRunContext, ErrorCode, NodeParam, NodeData};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};
use crate::seq::DelimitedSeq;

pub struct StreamBehaviour {
    is_streaming: bool,
    seq: DelimitedSeq,
}

impl StreamBehaviour {
    pub fn new(emit_seq: Box<dyn Seq>) -> StreamBehaviour {
        StreamBehaviour {
            is_streaming: false,
            seq: DelimitedSeq::new(
                Box::new(RepeatSeq::new(0, vec![]))
            )
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
        }
        else {
            /* Pop the in buffer */
            match context.in_buf.pop() {
                NodeData::Nil => {},
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

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        match args.remove("items") {
            Some(NodeParam::Seq(seq)) => self.seq = DelimitedSeq::new(seq),
            None => Err(ErrorCode::ArgumentRequired)?,
            _ => Err(ErrorCode::InvalidArgumentType)?,
        };

        Ok(())
    }
}
