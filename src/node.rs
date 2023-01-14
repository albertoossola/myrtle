use core::fmt;

use crate::var_store::VarStore;
use crate::behaviour::{Behaviour};

#[derive(Clone, Debug)]
pub enum NodeData {
  Int(i32),
  Float(f32),
  Bool(bool),
  Char(char),
  Byte(u8),
  End,
  Err,
  Ok,
  Pulse
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        NodeData::Int(i) => write!(f, "{}", i),
        NodeData::Float(fl) => write!(f, "{}", fl),
        NodeData::Bool(b) => write!(f, "{}", b),
        NodeData::Char(c) => write!(f, "{}", c),
        NodeData::Byte(b) => write!(f, "{}", b),
        NodeData::End => write!(f, "#end"),
        NodeData::Err => write!(f, "#err"),
        NodeData::Ok => write!(f, "#ok"),
        NodeData::Pulse => write!(f, "#pulse"),
    }
    }
}

#[derive(Clone)]
pub enum NodeParam {
  Int(i32),
  Float(f32),
  Bool(bool),
  Char(char),
  Byte(u8),
  Str(String)
}

pub struct Node {
  behaviour : Box<dyn Behaviour>,
  next : Option<Box<Node>>,
  in_buffer : Option<NodeData>
}

impl Node {
  pub fn step(&mut self, vars : &mut VarStore) -> () {
    let next_free = match &self.next {
      Some(node) => node.can_receive_data(),
      None => true
    }; 

    if next_free && self.in_buffer.is_some() {
      let step_result = self.behaviour.step(self.in_buffer.as_mut().unwrap().clone(), vars);

      if let Some(result_data) = step_result {
        match self.next.as_mut() {
          Some(node) => node.set_buffer(result_data),
          None => {}
        };
      }

      if !self.behaviour.is_working() {
        self.in_buffer = None;
      }
    }
    else {
      if let Some(node) = self.next.as_mut() {
        node.step(vars);
      }
    }
  }

  pub fn can_receive_data(&self) -> bool {
    self.in_buffer.is_none() && !self.behaviour.is_working()
  }

  pub fn reset(&mut self) {
    self.in_buffer = None;
    self.behaviour.reset();
  }

  pub fn set_param(&mut self, data : NodeParam) -> () {}
  pub fn set_buffer(&mut self, data : NodeData) -> () {
    self.in_buffer = Some(data);
  }

  pub fn set_next(&mut self, node : Box<Node>){
    self.next = Some(node);
  }

  pub fn new(behaviour : Box<dyn Behaviour>) -> Node {
    Node {
      next: None,
      in_buffer: None,
      behaviour
    }
  }

}