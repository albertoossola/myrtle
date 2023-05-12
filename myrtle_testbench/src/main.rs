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

use eframe::App;
use liblink::*;
use libmyrtle::*;

mod hal;
pub use hal::TestHal;
use ringbuffer::{
    ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferRead, RingBufferWrite,
};

mod gui;
use gui::GUI;

fn main() {
    println!("Myrtle Testbench - v0.1.0");

    println!("Initializing...");

    //Start gui
    let gui: Box<dyn App> = Box::new(GUI::default());

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2 { x: 400.0, y: 400.0 }),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native("Myrtle Testbench", native_options, Box::new(|cc| gui));
}
