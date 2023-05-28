use crate::NodeData;

pub trait Mask {
    fn is_done(&self) -> bool;
    fn push(&mut self, data : NodeData) -> Option<NodeData>;
    fn reset(&mut self) -> ();
}