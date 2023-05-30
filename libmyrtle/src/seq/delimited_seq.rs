use alloc::boxed::Box;
use crate::NodeData;
use crate::seq::delimited_seq::State::Closed;
use crate::seq::Seq;

#[derive(PartialEq)]
enum State {
    Starting,
    Enumerating,
    Closed
}

pub struct DelimitedSeq {
    state : State,
    wrapped : Box<dyn Seq>
}

impl DelimitedSeq {
    pub fn new(wrapped : Box<dyn Seq>) -> DelimitedSeq {
        DelimitedSeq {
            state: State::Starting,
            wrapped
        }
    }
}

impl Seq for DelimitedSeq {
    fn reset(&mut self) -> () {
        self.state = State::Starting;
        self.wrapped.reset();
    }

    fn poll(&mut self) -> Option<NodeData> {
        match self.state {
            State::Starting => {
                self.state = State::Enumerating;
                return Some(NodeData::Start);
            },
            State::Enumerating => {
                if self.wrapped.is_done() {
                    self.state = State::Closed;
                    return Some(NodeData::End);
                }

                return self.wrapped.poll();
            },
            State::Closed => None
        }
    }

    fn push(&mut self, data: NodeData) -> Option<NodeData> {
        if data == NodeData::Start {
            self.reset();
        }

        let to_return = self.wrapped.push(data);

        if self.wrapped.is_done() {
            self.state = State::Closed;
        }

        to_return
    }

    fn is_done(&self) -> bool {
        self.state == Closed || self.wrapped.is_done()
    }
}