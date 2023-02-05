use std::io::Empty;

use crate::{runtime::{Behaviour, Parametric, NodeParam}};

pub struct SetVarBehaviour {
  var : String
}

impl Behaviour for SetVarBehaviour {
    fn step(&mut self, data : crate::runtime::NodeData, vars : &mut crate::runtime::VarStore) -> Option<crate::runtime::NodeData> {
      vars.set(&self.var, data.clone());
      Some(data.clone())
    }

    fn is_working(&self) -> bool {
      false
    }

    fn reset(&mut self) -> () { }
}

impl Parametric for SetVarBehaviour {
    fn set_param(&mut self, param: &str, data : crate::runtime::NodeParam) -> () {
      match (param, data) {
        ("variable", NodeParam::Str(var_name)) => {
          self.var = var_name;
        },
        _ => {}
      }
    }

    fn get_params(&self) -> &[&str] {
      &["variable"]
    }
}

impl SetVarBehaviour {
  pub fn new() -> SetVarBehaviour {
    SetVarBehaviour { var: "".to_string() }
  } 
}