use crate::NodeData;

use super::Seq;

pub struct ConstSeq {
    enumerated: bool,
    pub value: NodeData,
}

impl ConstSeq {
    pub fn new(value: NodeData) -> ConstSeq {
        ConstSeq {
            enumerated: false,
            value,
        }
    }
}

impl Seq for ConstSeq {
    fn reset(&mut self) -> () {
        self.enumerated = false;
    }

    fn poll(&mut self) -> Option<NodeData> {
        match self.enumerated {
            true => None,
            false => {
                self.enumerated = true;
                Some(self.value)
            }
        }
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        self.enumerated = true;
        None
    }

    fn is_done(&self) -> bool {
        self.enumerated
    }
}