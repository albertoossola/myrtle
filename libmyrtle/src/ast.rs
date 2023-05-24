use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use crate::{NodeData, NodeParam};

pub struct ProgramAST {
    pub device: DeviceAST,
    pub machine: MachineAST,
}

pub struct DeviceAST {
    pub endpoints: BTreeMap<String, EndpointAST>,
}

pub struct MachineAST {
    pub name: String,
    pub states: Vec<StateAST>,
}

pub struct StateAST {
    pub name: String,
    pub flows: Vec<FlowAST>,
}

pub struct FlowAST {
    pub nodes: Vec<NodeAST>,
}

pub enum SeqAST {
    Const(NodeData),
    Repeat(i32, Vec<SeqAST>),
}

pub enum NodeParamAST {
    Base(NodeData),
    String(String),
    Seq(SeqAST),
}

pub struct NodeAST {
    pub kind: String,
    pub args: BTreeMap<String, NodeParamAST>,
}

//HW interface AST

pub struct EndpointAST {
    pub kind: String,
    pub args: BTreeMap<String, NodeParamAST>,
}
