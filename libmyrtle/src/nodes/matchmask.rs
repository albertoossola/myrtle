use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::{Behaviour, BehaviourRunContext, ErrorCode, NodeData, NodeArg};
use crate::mask::Mask;

pub struct MatchMask {
    mask : Box<dyn Mask>,
    in_data : Option<NodeData>
}

impl Behaviour for MatchMask {
    fn run(&mut self, context: BehaviourRunContext) -> () {
        match self.in_data {
            None => match context.in_buf.pop() {
                NodeData::Nil => {},
                data => self.in_data = Some(data)
            },
            Some(data) => match self.mask.push(data) {
                Some(data) => {
                    context.out_buf.push(data);

                    if self.mask.is_done() {
                        //TODO: Throw an event
                        self.mask.reset();
                    }

                    self.in_data = None;
                },
                None => {
                    self.mask.reset();
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.in_data = None;
        self.mask.reset();
    }

    fn init(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        Ok(())
    }
}