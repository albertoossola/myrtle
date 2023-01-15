pub struct LiteralNode {
  status : NodeStatus,
  value : NodeData
}

impl Behaviour for LiteralNode {
    fn step(&mut self, data : crate::node::NodeData, vars : &mut crate::var_store::VarStore) -> crate::node::NodeStatus {
      match data {
        NodeData::Pulse => { },
        _ => { self.status = NodeStatus::Full }
      };

      self.status
    }

    fn set_param(&mut self, data : crate::node::NodeParam) -> () {
       
    }

    fn get_status(&self) -> crate::node::NodeStatus {
        self.status
    }

    fn pop_buffer(&mut self) -> Option<crate::node::NodeData> {
      let out = match self.status {
        NodeStatus::Full => Some(self.value.clone()),
        _ => None
      };

      self.status = NodeStatus::Idle;

      out
    }

    fn reset(&mut self) {
      self.status = NodeStatus::Idle;
    }
}

impl LiteralNode {
  pub fn new(value : NodeData) -> LiteralNode {
    LiteralNode { status: NodeStatus::Idle, value }
  }
}