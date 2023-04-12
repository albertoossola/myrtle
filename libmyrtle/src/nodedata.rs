use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug)]
pub enum NodeData {
    Int(i32),
    Bool(bool),
    Char(char),
    Nil,
}

pub enum NodeParam {
    Base(NodeData),
    String(String),
    Seq(Vec<NodeData>),
}
