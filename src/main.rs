mod runtime;
mod parser;

use chumsky::Parser;
use runtime::*;
use parser::*;

use std::{thread, time::Duration};

fn main() {
    let source = std::fs::read_to_string("./test_machine.myr").unwrap();

    let machine_parser = parser::parser();

    match machine_parser.parse(source) {
        Ok(mut machine) => {
            let mut vars = VarStore::new();

            loop {
                machine.set_buffer(NodeData::Pulse);
        
                machine.step(&mut vars);
                thread::sleep(Duration::from_millis(1));
            }
        },
        Err(errors) => {
            for error in errors.iter() {
                println!("{}", error);
            }
        }
    }
}
    