use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::{Behaviour, BehaviourRunContext, ErrorCode, NodeData, NodeArg, VariableSet};

pub struct EaseBehaviour {
    pub target_value: f32,
    pub rate: f32,
    current_value : f32,
    last_run_us: u64
}

impl EaseBehaviour {
    pub fn new() -> EaseBehaviour {
        EaseBehaviour {
            rate: 1f32,
            last_run_us: 0,
            target_value: 0.0,
            current_value: 0.0
        }
    }
}

impl Behaviour for EaseBehaviour {
    fn is_working(&self) -> bool { true }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        if self.last_run_us == 0 {
            self.last_run_us = context.current_ticks_us;
        }

        if context.current_ticks_us - self.last_run_us == 0 {
            return;
        }

        match context.in_buf.pop() {
            NodeData::Float(f) => self.target_value = f,
            NodeData::Int(i) => self.target_value = i as f32,
            _ => {}
        };

        //Multiply the rate by the delta time
        let dt = ((context.current_ticks_us - self.last_run_us) as f32 / 1000000f32);
        let dv = (self.rate * dt).clamp(0f32, 1f32);

        if self.current_value <= self.target_value {
            self.current_value = (self.current_value + dv).clamp(self.current_value, self.target_value);
        }
        else {
            self.current_value = (self.current_value - dv).clamp(self.target_value, self.current_value);
        }

        self.last_run_us = context.current_ticks_us;

        context.out_buf.push(NodeData::Float(self.current_value));
    }

    fn reset(&mut self) -> () {
        self.target_value = 0.0;
        self.current_value = self.target_value;
        self.last_run_us = 0;
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        match args.remove("rate") {
            Some(NodeArg::Base(NodeData::Float(f))) if (f >= 0.0) => {
                self.rate = f;
                return Ok(());
            },
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        }
    }
}