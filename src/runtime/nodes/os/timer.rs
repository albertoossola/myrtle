use std::time::{Instant, Duration};
use super::*;

pub struct TimerBehaviour {
  last_tick : Instant,
  interval : Duration
}

impl Behaviour for TimerBehaviour {
  fn step(&mut self, data : NodeData, vars : &mut VarStore) -> Option<NodeData> {
    if self.last_tick.elapsed() > self.interval {
      self.last_tick = Instant::now();
      return Some(NodeData::Pulse);
    }

    None
  }

  fn is_working(&self) -> bool {
    false
  }

  fn reset(&mut self) {
    self.last_tick = Instant::now();
  }
}

impl Parametric for TimerBehaviour {
    fn set_param(&mut self, param: &str, data : NodeParam) -> () {
      match (param, data) {
        ("interval", NodeParam::Int(ms)) => {
          self.interval = Duration::from_millis(ms as u64);
        },
        _ => {}
      }
    }

    fn get_params(&self) -> &[&str] {
      &["interval"]
    }
}

impl TimerBehaviour {
  pub fn new() -> TimerBehaviour {
    TimerBehaviour {
      interval: Duration::from_millis(1000),
      last_tick: Instant::now()
    }
  }
}