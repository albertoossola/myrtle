extern crate liblink;
extern crate libmyrtle;

mod listener;
use listener::*;

use std::{
    alloc::System,
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, UdpSocket},
    time::{SystemTime, UNIX_EPOCH},
};
use std::thread::sleep;
use std::time::Duration;

use liblink::*;
use libmyrtle::*;

mod hal;
pub use hal::TestHal;
use ringbuffer::{
    ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferRead, RingBufferWrite,
};

mod app;
use app::App;

fn main() {
    println!("Myrtle for Raspberry Pi - v0.1.0");

    println!("Initializing...");

    //Start gui
    let mut app = App::default();

    loop {
        app.update();
        sleep(Duration::from_micros(500));
    }
}
