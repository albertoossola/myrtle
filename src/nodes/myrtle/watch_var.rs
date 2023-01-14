use crate::{node::{Node, NodeStatus, NodeData}, var_store::Var};

pub struct WatchVarEvent {
  var : String,
  status : NodeStatus,
  latest_var_count : i32,
  prev_var_count : i32,
  out_buffer : NodeData
}

impl Node for WatchVarEvent {
  fn step(&mut self, _ : crate::node::NodeData, vars : &mut crate::var_store::VarStore) -> crate::node::NodeStatus {
    match vars.get(&self.var) {
      Some(Var(value, count)) => {
        self.out_buffer = value.clone();
        self.latest_var_count = *count;
        self.status = NodeStatus::Full;
      },
      None => {}
    };

    self.status
  }

  fn set_param(&mut self, data : crate::node::NodeParam) -> () {
    
  }

  fn get_status(&self) -> crate::node::NodeStatus {
    if self.latest_var_count == self.prev_var_count {
      NodeStatus::Idle
    }
    else {
      NodeStatus::Full
    }
  }

  fn pop_buffer(&mut self) -> Option<crate::node::NodeData> {
    if self.latest_var_count == self.prev_var_count {
      None
    }
    else {
      self.prev_var_count = self.latest_var_count;
      self.status = NodeStatus::Idle;

      Some(self.out_buffer.clone())
    }
  }

  fn reset(&mut self) {
    self.status = NodeStatus::Idle;
    self.out_buffer = NodeData::Pulse;
  }
}

impl WatchVarEvent {
  pub fn new(var : &str) -> WatchVarEvent {
    WatchVarEvent {
      latest_var_count: 0,
      prev_var_count: 0,
      status: NodeStatus::Idle,
      out_buffer: NodeData::Pulse,
      var: var.to_string()
    }
  }
}