use crate::NodeData;

pub trait Seq {
    fn reset(&mut self) -> ();
    fn poll(&mut self) -> Option<NodeData>;
    fn is_done(&self) -> bool;
}
