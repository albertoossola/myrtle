use super::super::super::*;

pub struct CounterNode {
  count : i32
}

impl Behaviour for CounterNode {
  fn step(&mut self, data : NodeData, vars : &mut VarStore) -> Option<NodeData> {
    match data {
      NodeData::Int(amount) => { self.count += amount; },
      _ => { self.count += 1 }
    };

    Some(NodeData::Int(self.count))
  }

  fn is_working(&self) -> bool {
    false
  }

  fn reset(&mut self) -> () {
    self.count = 0;
  }
}

impl Parametric for CounterNode {
  fn set_param(&mut self, param: &str, data : NodeParam) -> () {
  }

fn get_params(&self) -> &[&str] {
        todo!()
    }
}

impl CounterNode {
  pub fn new() -> CounterNode {
    CounterNode { count: 0 }
  }
}