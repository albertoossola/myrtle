use super::*;

pub trait Behaviour {
  fn step(&mut self, data : NodeData, vars : &mut VarStore) -> Option<NodeData>;
  fn is_working(&self) -> bool;
  fn reset(&mut self) -> ();
}