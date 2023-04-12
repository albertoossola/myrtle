extern crate libmyrtle;

use std::{
    alloc::System,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use libmyrtle::*;

mod hal;
pub use hal::TestHal;

fn main() {
    println!("Myrtle Testbench - v0.1.0");

    println!("Initializing HAL...");

    let mut hal_instance = hal::TestHal::init();

    println!("Loading program from file...");

    let source: String = String::from_utf8(fs::read("./main.myr").unwrap()).unwrap();

    let mut machine_ast = parse_machine(source.as_str()).unwrap().1;

    let mut machine = make_machine(&mut machine_ast).unwrap();

    //Add a led to the variables
    machine.variables.insert(
        String::from("led"),
        Symbol::new(hal_instance.set_push_pull_pin(25)),
    );

    loop {
        let context = MachineRunContext {
            current_ticks: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        machine.run(context)
    }
}
