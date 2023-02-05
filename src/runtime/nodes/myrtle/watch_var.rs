use crate::runtime::{Behaviour, NodeData, Parametric, NodeParam, Var};

pub struct WatchVarBehaviour {
  var : String,
  prev_var_count : i32,
}

impl Behaviour for WatchVarBehaviour {
    fn step(&mut self, data : crate::runtime::NodeData, vars : &mut crate::runtime::VarStore) -> Option<crate::runtime::NodeData> {
      match vars.get(self.var.as_str()) {
        Some(Var(data, count)) if *count > self.prev_var_count => {
          self.prev_var_count = *count;
          Some(data.clone())
        },
        _ => None
      }
    }

    fn is_working(&self) -> bool {
      false
    }

    fn reset(&mut self) -> () {
      self.prev_var_count = 0;
    }
}

impl Parametric for WatchVarBehaviour {
    fn set_param(&mut self, param: &str, data : crate::runtime::NodeParam) -> () {
      match (param, data) {
        ("variable", NodeParam::Str(s)) => {
          self.var = s;
        },
        _ => { }
      }
    }

    fn get_params(&self) -> &[&str] {
      &["variable"]
    }
}

impl WatchVarBehaviour {
  pub fn new() -> WatchVarBehaviour {
    WatchVarBehaviour {
      prev_var_count: 0,
      var: "".to_string()
    }
  }
}