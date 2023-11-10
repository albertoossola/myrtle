extern crate liblink;
extern crate libmyrtle;

use std::thread::sleep;
use std::time::Duration;

use libmyrtle::*;

mod hal;
mod tcp_ip_command_source;
mod file_system_channel;
mod stdio_shell_io;

pub use hal::RaspberryPiHal;

use myrtle_instance::MyrtleInstance;
use crate::stdio_shell_io::StdioShellIO;

fn main() {
    println!("Myrtle for Raspberry Pi - v0.1.0");

    println!("Initializing...");

    let hardware_layer = RaspberryPiHal::new();
    let shell_io = StdioShellIO::new();

    let mut instance = MyrtleInstance::new(
        Box::new(hardware_layer),
        Box::new(shell_io)
    );

    println!("Ready to go!");

    loop {
        instance.step();
        sleep(Duration::from_micros(500));
    }
}
