use alloc::{boxed::Box, string::String, vec::Vec};

use crate::seq::Seq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeData {
    Int(i32),
    Float(f32),
    Bool(bool),
    Char(char),
    Start,
    End,
    Err,
    Nil,
}

pub enum NodeArg {
    Base(NodeData),
    String(String),
    Seq(Box<dyn Seq>),
}
