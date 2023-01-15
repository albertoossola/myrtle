use std::time::{Instant, Duration};
use super::*;

pub struct TimerNode {
  last_tick : Instant,
  interval : Duration
}

impl Behaviour for TimerNode {
  fn step(&mut self, data : NodeData, vars : &mut VarStore) -> Option<NodeData> {
    if self.last_tick.elapsed() > self.interval {
      self.last_tick = Instant::now();
      return Some(NodeData::Pulse);
    }

    None
  }

  fn reset(&mut self) {
    self.last_tick = Instant::now();
  }

  fn is_working(&self) -> bool {
    false
  }
}

impl TimerNode {
  pub fn new(interval : Duration) -> TimerNode {
    TimerNode {
      interval,
      last_tick: Instant::now()
    }
  }
}