use alloc::{boxed::Box, vec::Vec};
use crate::NodeData;

use super::Seq;

pub struct RepeatSeq {
    pub wrapped: Box<dyn Seq>,
    pub iterations: i32,
    pub counter: i32,
}

impl RepeatSeq {
    pub fn new(iterations: i32, wrapped: Box<dyn Seq>) -> RepeatSeq {
        RepeatSeq {
            wrapped,
            iterations,
            counter: 0
        }
    }
}

impl Seq for RepeatSeq {
    fn reset(&mut self) -> () {
        self.counter = 0;
        self.wrapped.reset();
    }

    fn poll(&mut self) -> Option<crate::NodeData> {
        if self.is_done() {
            return None;
        }

        let to_return = self.wrapped.poll();

        if self.wrapped.is_done() {
            self.wrapped.reset();
            self.counter += 1;
        }

        return to_return;
    }

    fn push(&mut self, data : NodeData) -> Option<crate::NodeData> {
        if self.is_done() {
            return None;
        }

        let to_return = self.wrapped.push(data);

        if self.wrapped.is_done() {
            self.wrapped.reset();
            self.counter += 1;
        }

        return to_return;
    }

    fn is_done(&self) -> bool {
        if self.wrapped.is_done() || self.counter >= self.iterations {
            return true;
        }

        false
    }
}
