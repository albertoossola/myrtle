use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::NodeParam;

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

pub struct NodeAST {
    pub kind: String,
    pub args: BTreeMap<String, NodeParam>,
}
