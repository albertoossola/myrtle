use alloc::{boxed::Box, collections::BTreeMap, string::String};

use crate::{
    Behaviour, BehaviourRunContext, ErrorCode, MemoryDataSource, NodeData, NodeParam, Symbol,
};

enum State {
    Idle,
    Opening,
    WritingSingle,
    WaitingForStreamData,
    Closing,
}

pub struct SetVarBehaviour {
    var_name: String,
    state: State,
}

impl SetVarBehaviour {
    pub fn new(var_name: String) -> SetVarBehaviour {
        SetVarBehaviour {
            var_name,
            state: State::Idle,
        }
    }
}

impl Behaviour for SetVarBehaviour {
    fn is_working(&self) -> bool {
        match self.state {
            State::Idle => false,
            State::WaitingForStreamData => false,
            State::Opening => true,
            State::Closing => true,
            State::WritingSingle => true,
        }
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        let var: &mut Symbol = match context.machine_vars.get_mut(&self.var_name) {
            Some(v) => v,
            None => {
                context.machine_vars.insert(
                    self.var_name.clone(),
                    Symbol::new(Box::new(MemoryDataSource::new())),
                );

                context.machine_vars.get_mut(&self.var_name).unwrap()
            }
        };

        match self.state {
            State::Idle => {
                if context.in_buf.is_full() {
                    self.state = State::Opening;
                }
            }
            State::Opening => {
                if var.can_open() {
                    var.open();

                    match context.in_buf.peek() {
                        NodeData::Start => {
                            self.state = State::WaitingForStreamData;
                            context.in_buf.pop();
                        }
                        _ => {
                            self.state = State::WritingSingle;
                        }
                    }
                }
            }
            State::WritingSingle => {
                if var.can_push() {
                    let value = context.in_buf.pop();
                    var.push(value);
                    self.state = State::Closing;
                }
            }
            State::WaitingForStreamData => match context.in_buf.peek() {
                NodeData::End => {
                    context.in_buf.pop();
                    self.state = State::Closing;
                }
                NodeData::Nil => {}
                _ => {
                    if var.can_push() {
                        var.push(context.in_buf.pop());
                    }
                }
            },
            State::Closing => {
                var.close();
                self.state = State::Idle;
            }
        }
    }

    fn reset(&mut self) -> () {
        todo!()
    }

    fn init(&mut self, args: &mut BTreeMap<String, NodeParam>) -> Result<(), ErrorCode> {
        match args.remove("var") {
            Some(NodeParam::String(var)) => self.var_name = var,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }
}
