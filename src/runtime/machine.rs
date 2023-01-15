use super::*;

pub struct Machine {
  states : Vec<State>,
  cursor : usize,
  out_var_cnt : i32
}

impl Behaviour for Machine {
  fn step(&mut self, data : NodeData, vars : &mut VarStore) -> Option<NodeData> {
    match vars.get("@state") {
      Some(Var(NodeData::Int(value), _)) => {
        let clamped_value = (*value as usize).clamp(0, self.states.len() - 1); 
        
        //Reset the new state on change
        if clamped_value != self.cursor {
          self.states.get_mut(clamped_value).unwrap().reset();
        }

        self.cursor = clamped_value;
      },
      _ => {}
    }
    
    {
      let cur_state = self.states.get_mut(self.cursor).unwrap();
      cur_state.step(vars);

      match vars.get("@out") {
        Some(Var(out_val, cnt)) if *cnt != self.out_var_cnt => {
          self.out_var_cnt = *cnt;
          return Some(out_val.clone());
        },
        _ => {}
      };
    }

    return None;
  }

  fn reset(&mut self) {
    self.states.iter_mut().for_each(|s| s.reset());
  }

  fn is_working(&self) -> bool {
    //TODO: it's not working when the input data was passed to all flows
    //That might react to the input event

    false
  }
}

impl Machine {
  pub fn new(states : Vec<State>) -> Machine {
    Machine {
      states,
      cursor : 0,
      out_var_cnt : 0
    }
  }
}