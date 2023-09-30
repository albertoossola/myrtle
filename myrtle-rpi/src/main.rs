extern crate liblink;
extern crate libmyrtle;

use std::thread::sleep;
use std::time::Duration;



use libmyrtle::*;

mod hal;
mod tcp_ip_command_source;
mod file_system_channel;

pub use hal::RaspberryPiHal;
use libmyrtle::interface::channels::{Channel, MemoryBufferChannel};

use myrtle_instance::MyrtleInstance;
use crate::file_system_channel::FileSystemChannel;
use crate::tcp_ip_command_source::TcpIpCommandSource;

fn main() {
    println!("Myrtle for Raspberry Pi - v0.1.0");

    println!("Initializing...");

    let command_source = TcpIpCommandSource::new("0.0.0.0:42069").unwrap();
    let hardware_layer = RaspberryPiHal::new();

    let interface_channels : Vec<Box<dyn Channel>> = vec! [
        Box::new(MemoryBufferChannel::new()),
        Box::new(FileSystemChannel::new("./boot.myr", 1024 * 5))
    ];

    


    let mut instance = MyrtleInstance::new(
        Box::new(hardware_layer),
        Box::new(command_source),
        interface_channels
    );

    println!("Loading stored program...");

    //Run code from the persistent stored channel
    //instance.run_from_channel(0);

    println!("Ready to go!");

    loop {
        instance.step();
        sleep(Duration::from_micros(500));
    }
}
