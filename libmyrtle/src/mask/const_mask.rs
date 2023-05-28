use crate::mask::mask::Mask;
use crate::NodeData;

pub struct ConstMask {
    value : NodeData,
    done : bool
}

impl Mask for ConstMask {
    fn is_done(&self) -> bool {
        self.done
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        if self.value == data {
            self.done = true;
            return Some(NodeData::Nil);
        }

        return None;
    }

    fn reset(&mut self) -> () {
        self.done = false;
    }
}