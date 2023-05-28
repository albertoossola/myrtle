use crate::mask::mask::Mask;
use crate::NodeData;

pub struct ByteMask {
    done: bool
}

impl Mask for ByteMask {
    fn is_done(&self) -> bool {
        return self.done;
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        return match data {
            NodeData::Int(i) => {
                self.done = true;
                Some(NodeData::Int(i & 0xff))
            }
            _ => {
                None
            }
        }
    }

    fn reset(&mut self) -> () {
        self.done = false;
    }
}