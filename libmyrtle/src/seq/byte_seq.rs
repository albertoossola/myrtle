use crate::seq::Seq;
use crate::NodeData;

pub struct ByteSeq {
    done: bool,
}

impl ByteSeq {
    pub fn new() -> ByteSeq {
        ByteSeq { done: false }
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
            _ => None,
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use crate::{seq::Seq, NodeData};

    use super::ByteSeq;

    #[test]
    pub fn filter_byte() {
        let mut seq = ByteSeq::new();

        let filtered = seq.push(NodeData::Char('a'));

        let char_has_passed = match filtered {
            Some(NodeData::Int(0x61)) => true,
            _ => false,
        };

        assert!(char_has_passed);
        assert!(seq.is_done() == true)
    }

    #[test]
    pub fn filter_int() {
        let mut seq = ByteSeq::new();

        let filtered = seq.push(NodeData::Int(0xFF97));

        let int_has_passed = match filtered {
            Some(NodeData::Int(0x97)) => true,
            _ => false,
        };

        assert!(int_has_passed);
        assert!(seq.is_done() == true)
    }

    #[test]
    pub fn filter_other() {
        let mut seq = ByteSeq::new();

        let filtered = seq.push(NodeData::Float(0.5));

        let has_failed = match filtered {
            None => true,
            _ => false,
        };

        assert!(has_failed);
        assert!(seq.is_done() == true)
    }
}
