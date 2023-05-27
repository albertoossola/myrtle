use alloc::{boxed::Box, vec::Vec};

use super::Seq;

pub struct RepeatSeq {
    pub wrapped: Vec<Box<dyn Seq>>,
    pub iterations: i32,
    pub counter: i32,
    pub index: usize,
}

impl RepeatSeq {
    pub fn new(iterations: i32, wrapped: Vec<Box<dyn Seq>>) -> RepeatSeq {
        RepeatSeq {
            wrapped,
            iterations,
            counter: 0,
            index: 0,
        }
    }
}

impl Seq for RepeatSeq {
    fn reset(&mut self) -> () {
        self.counter = 0;
        self.index = 0;
        self.wrapped.iter_mut().for_each(|w| w.reset());
    }

    fn poll(&mut self) -> Option<crate::NodeData> {
        if self.is_done() {
            return None;
        }

        let current_seq = &mut self.wrapped[self.index];

        let polled = current_seq.poll();

        if current_seq.is_done() {
            current_seq.reset();
            self.index += 1;
        }

        if self.index >= self.wrapped.len() {
            self.index = 0;
            self.counter += 1;
        }

        return polled;
    }

    fn is_done(&self) -> bool {
        if self.wrapped.is_empty() || self.counter >= self.iterations {
            return true;
        }

        false
    }
}
