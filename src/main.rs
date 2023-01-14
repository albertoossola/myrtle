mod node;
mod nodes;
mod machine;
mod state;
mod var_store;
mod behaviour; 
mod flow_builder;

use std::{thread, time::Duration};

use behaviour::Behaviour;
use flow_builder::FlowBuilder;
use nodes::{ os::*, utils::* };

use node::*;
use machine::*;
use state::*;
use var_store::VarStore;

fn main() {    
    let mut builder = FlowBuilder::new(Box::new(TimerNode::new(Duration::from_micros(10))));
    builder.append(Box::new(CounterNode::new()));
    builder.append(Box::new(PrintNode {}));
    
    let f_a = builder.build();

    let s_a = State::new(vec![*f_a]);
    let mut m = Machine::new(vec![s_a]);

    let mut vars = VarStore::new();

    loop {
        m.step(NodeData::Pulse, &mut vars);
        //thread::sleep(Duration::from_millis(1));
    }
}
    