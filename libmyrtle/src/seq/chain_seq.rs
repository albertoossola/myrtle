use alloc::{boxed::Box, vec::Vec};
use crate::NodeData;

use super::Seq;

pub struct ChainSeq {
    pub wrapped: Vec<Box<dyn Seq>>,
    pub index: usize,
}

impl ChainSeq {
    pub fn new(wrapped: Vec<Box<dyn Seq>>) -> ChainSeq {
        ChainSeq {
            wrapped,
            index: 0,
        }
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
