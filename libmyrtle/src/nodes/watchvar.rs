/* The WatchVar behaviour, moved from node.rs */

use alloc::{string::String, collections::BTreeMap, boxed::Box};

use crate::{Behaviour, BehaviourRunContext, NodeData, VariableSet, NodeArg, ErrorCode, MemoryDataSource, Symbol};

/* WatchVar */
pub struct WatchVarBehaviour {
    var_name: String,
    listener_id: i32,
}

impl WatchVarBehaviour {
    pub fn new() -> WatchVarBehaviour {
        WatchVarBehaviour {
            var_name: String::from(""),
            listener_id: -1,
        }
    }
}

impl Behaviour for WatchVarBehaviour {
    fn is_working(&self) -> bool {
        false
    }

    fn run(&mut self, context: BehaviourRunContext) -> () {
        let var = context.machine_vars.get_mut(&self.var_name).unwrap();
        let polled = var.poll(self.listener_id);

        match polled {
            NodeData::Nil => {}
            _ => {
                context.out_buf.push(polled);
            }
        };
    }

    fn on_state_change(&mut self, vars: &mut VariableSet) -> () {
        if self.listener_id == -1 {
            return;
        }

        vars.get_mut(&self.var_name)
            .and_then(|v| { v.suspend_listener(self.listener_id); return Some(()) });
    }

    fn on_state_enter(&mut self, vars: &mut VariableSet) -> () {
        if self.listener_id == -1 {
            return;
        }

        vars.get_mut(&self.var_name)
            .and_then(|v| { v.activate_listener(self.listener_id); return Some(()) });
        
    }

    fn reset(&mut self) -> () {
        //TODO: Unregister listener
    }

    fn set_args(&mut self, args: &mut BTreeMap<String, NodeArg>) -> Result<(), ErrorCode> {
        self.listener_id = -1;

        match args.remove("var") {
            Some(NodeArg::String(var)) => self.var_name = var,
            Some(_) => Err(ErrorCode::InvalidArgumentType)?,
            None => Err(ErrorCode::ArgumentRequired)?,
        };

        Ok(())
    }

    fn init(&mut self, vars: &mut VariableSet) -> () {
        if !vars.contains_key(&self.var_name) {
            vars.insert(
                self.var_name.clone(),
                Symbol::new(Box::new(MemoryDataSource::new())),
            );
        }

        let var = vars.get_mut(&self.var_name).unwrap();

        self.listener_id = var.register_listener();
    }
}
