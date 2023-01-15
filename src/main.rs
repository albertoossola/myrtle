#[macro_use]
extern crate pest_derive;

mod runtime;
mod parser;

use runtime::*;
use parser::*;

use std::{thread, time::Duration, io};

fn main() {
    let source = std::io::stdin()
        .lines()
        .fold(
            String::new(), 
            |mut a, f| { a.push_str(f.unwrap().as_str()); a})
        ;

    let mut machine = parse(source.as_str());
    let mut vars = VarStore::new();

    loop {
        machine.step(NodeData::Pulse, &mut vars);
        thread::sleep(Duration::from_millis(1));
    }
}
    