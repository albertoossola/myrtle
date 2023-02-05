use crate::runtime::Behaviour;
use crate::runtime::Node;

use super::myrtle;
use super::utils;
use super::os;

pub fn make_node(id : &str) -> Option<crate::runtime::Node> {
  let behaviour : Option<Box<dyn Behaviour>> = match id {
    "myrtle::literal" => Some(Box::new(myrtle::LiteralNode::new())),
    "myrtle::set_var" => Some(Box::new(myrtle::SetVarBehaviour::new())),
    "myrtle::watch_var" => Some(Box::new(myrtle::WatchVarBehaviour::new())),
    "utils::counter" => Some(Box::new(utils::CounterNode::new())),
    "os::print" => Some(Box::new(os::PrintBehaviour::new())),
    "os::timer" => Some(Box::new(os::TimerBehaviour::new())),
    _ => None
  };

  behaviour.map(|b| Node::new(b))
}