use crate::NodeData;
use alloc::{boxed::Box, vec::Vec};

use super::Seq;

pub struct ChainSeq {
    pub wrapped: Vec<Box<dyn Seq>>,
    pub index: usize,
}

impl ChainSeq {
    pub fn new(wrapped: Vec<Box<dyn Seq>>) -> ChainSeq {
        ChainSeq { wrapped, index: 0 }
    }
}

impl Seq for ChainSeq {
    fn reset(&mut self) -> () {
        self.index = 0;
        self.wrapped.iter_mut().for_each(|w| w.reset());
    }

    fn poll(&mut self) -> Option<NodeData> {
        if self.index >= self.wrapped.len() {
            return None;
        }

        let current = &mut self.wrapped[self.index];
        let polled = current.poll();

        if current.is_done() {
            self.index += 1;
        }

        return polled;
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        if self.index >= self.wrapped.len() {
            return None;
        }

        let current = &mut self.wrapped[self.index];
        let polled = current.push(data);

        if current.is_done() {
            self.index += 1;
        }

        return polled;
    }

    fn is_done(&self) -> bool {
        self.index >= self.wrapped.len()
    }
}

#[cfg(test)]
mod tests {
    use alloc::{boxed::Box, string::String, vec, vec::Vec};

    use crate::{
        seq::{ByteSeq, ConstSeq, Seq},
        NodeData,
    };

    use super::ChainSeq;

    #[test]
    fn filter_fixed() {
        let content: Vec<Box<dyn Seq>> = vec![
            Box::new(ConstSeq::new(crate::NodeData::Char('H'))),
            Box::new(ConstSeq::new(crate::NodeData::Char('i'))),
            Box::new(ConstSeq::new(crate::NodeData::Char('!'))),
        ];

        let mut seq = ChainSeq::new(content);

        for c in "Hi!".chars() {
            let push_result = seq.push(crate::NodeData::Char(c));

            let is_success = match push_result {
                Some(NodeData::Nil) => true,
                _ => false,
            };

            assert!(is_success);
        }
    }

    #[test]
    fn filter_middle() {
        let content: Vec<Box<dyn Seq>> = vec![
            Box::new(ConstSeq::new(crate::NodeData::Char('H'))),
            Box::new(ConstSeq::new(crate::NodeData::Char('i'))),
            Box::new(ConstSeq::new(crate::NodeData::Char('!'))),
            Box::new(ByteSeq::new()),
            Box::new(ByteSeq::new()),
        ];

        let mut seq = ChainSeq::new(content);

        let mut output = String::new();

        for letter in "Hi!XY".chars() {
            let push_result = seq.push(crate::NodeData::Char(letter));

            match push_result {
                Some(NodeData::Int(i)) => {
                    output.push(char::from_u32(i as u32).unwrap());
                }
                None => panic!(),
                _ => {}
            };
        }

        assert_eq!(output, "XY");
    }
}
