use crate::NodeData;
use crate::seq::Seq;

pub struct ByteSeq {
    done : bool
}

impl ByteSeq {
    pub fn new() -> ByteSeq {
        ByteSeq {
            done: false
        }
    }
}

impl Seq for ByteSeq {
    fn reset(&mut self) -> () {
        self.done = false;
    }

    fn poll(&mut self) -> Option<NodeData> {
        None
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        self.done = true;

        match data {
            NodeData::Char(c) => Some(NodeData::Int((c as u8 & 0xff) as i32)),
            NodeData::Int(i) => Some(NodeData::Int(i & 0xff)),
            _ => None
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}