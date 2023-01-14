
use crate::{node::{NodeData}, behaviour::Behaviour};

pub struct PrintNode {
}

impl Behaviour for PrintNode {
    fn step(
        &mut self,
        data: NodeData,
        vars: &mut crate::var_store::VarStore,
    ) -> Option<NodeData> {
        println!("{}", data);
        Some(data)
    }

    fn reset(&mut self) { }

    fn is_working(&self) -> bool { false }
}