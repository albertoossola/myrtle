use super::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Var(pub NodeData, pub i32);

pub struct VarStore {
  data : HashMap<String, Var>
}

impl VarStore {
  pub fn new() -> VarStore {
    VarStore {
      data : HashMap::new()
    }
  }

  pub fn get(&self, var : &str) -> Option<&Var> {
    self.data.get(var)
  }

  pub fn set(&mut self, var : &str, val : NodeData) -> i32 {
    
    let var_opt = self.data.get(var).clone();

    match var_opt {
      Some(Var(_, wcnt)) => {
        let new_count = wcnt + 1 % i32::MAX;

        self.data.insert(var.to_string(), Var(val, new_count));
        
        new_count
      },
      None => {
        self.data.insert(var.to_string(), Var(val, 1));
        
        1
      }
    }
  }
}