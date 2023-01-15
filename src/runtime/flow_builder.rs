use super::*;

pub struct FlowBuilder {
  nodes : Vec<Box<dyn Behaviour>>
}

impl FlowBuilder {
  pub fn append(&mut self, node : Box<dyn Behaviour>) -> &mut FlowBuilder{
    self.nodes.push(node);

    return self;
  }

  pub fn build(&mut self) -> Box<Node> {
    let last = self.nodes.pop().unwrap();
    let mut built = Box::new(Node::new(last));

    while !self.nodes.is_empty() {
      let popped = self.nodes.pop().unwrap();
      let mut new_node = Box::new(Node::new(popped));
      new_node.set_next(built);

      built = new_node;
    }

    built
  }

  pub fn new() -> FlowBuilder {
    FlowBuilder { nodes: vec![] }
  }
}