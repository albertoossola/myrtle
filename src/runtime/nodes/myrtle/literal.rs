use crate::runtime::{NodeData, Behaviour, Parametric, NodeParam};

pub struct LiteralNode {
  value : NodeData
}

impl Behaviour for LiteralNode {
  fn step(&mut self, data : NodeData, vars : &mut crate::runtime::VarStore) -> Option<NodeData> {
    Some(self.value.clone())
  }

  fn is_working(&self) -> bool { false }

  fn reset(&mut self) -> () { }
}

impl Parametric for LiteralNode {
  fn set_param(&mut self, param: &str, data : crate::runtime::NodeParam) -> () {
    match (param, data) {
      ("value", NodeParam::Str(_)) => {},
      ("value", NodeParam::Int(i)) => { self.value = NodeData::Int(i) }, 
      ("value", NodeParam::Char(c)) => { self.value = NodeData::Char(c) }, 
      ("value", NodeParam::Byte(b)) => { self.value = NodeData::Byte(b) }, 
      ("value", NodeParam::Bool(b)) => { self.value = NodeData::Bool(b) }, 
      ("value", NodeParam::Float(f)) => { self.value = NodeData::Float(f) }, 
      _ => {}
    }
  }

  fn get_params(&self) -> &[&str] {
      todo!()
  }
}

impl LiteralNode {
  pub fn new() -> LiteralNode {
    LiteralNode { value: NodeData::Pulse }
  }
}