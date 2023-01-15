use std::io::Empty;

use crate::node::Node;

pub struct SetVarNode {
  var : String
}

impl Node for SetVarNode {
    fn step(&mut self, data : crate::node::NodeData, vars : &mut crate::var_store::VarStore) -> crate::node::NodeStatus {
      match data {
        crate::node::NodeData::Pulse => {},
        _ =>  { vars.set(&self.var, data); }
      };

      crate::node::NodeStatus::Idle
    }

    fn set_param(&mut self, data : crate::node::NodeParam) -> () {
      
    }

    fn get_status(&self) -> crate::node::NodeStatus {
      crate::node::NodeStatus::Idle
    }

    fn pop_buffer(&mut self) -> Option<crate::node::NodeData> {
      None
    }

    fn reset(&mut self) {
    }
}

impl SetVarNode {
  pub fn new(var : &str) -> SetVarNode {
    SetVarNode { var: var.to_string() }
  } 
}