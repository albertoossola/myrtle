use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Output};
use libmyrtle::{DataSource, NodeData};

pub struct Uart {
    tx_pin: i32,
    rx_pin: i32,
    baud: i32,
    handle: Option<File>
}

impl Uart {
    pub fn new(tx_pin: i32, rx_pin: i32, baud: i32) -> Uart {
        println!("Setting UART baud rate: {}", baud);

        let _ = Command::new("stty")
            .arg("-F")
            .arg("/dev/ttyS0")
            .arg(baud.to_string())
            .output()
            .map_err(|e| println!("Failed to set baud rate: {}", e));

        Uart {
            baud,
            tx_pin,
            rx_pin,
            handle: None
        }
    }
}

impl DataSource for Uart {
    fn poll(&mut self) -> NodeData {
        match self.handle.as_mut() {
            Some(mut f) => {
                let mut buf: [u8; 1] = [0];
                match f.read(&mut buf) {
                    Ok(len) if len > 0 => NodeData::Int(buf[0] as i32),
                    _ => NodeData::Nil
                }
            },
            None => NodeData::Nil
        }
    }

    fn can_push(&self) -> bool { true }

    fn push(&mut self, data: NodeData) -> () {
        let byte : Option<u8> = match data {
            NodeData::Int(int) => Some((int & 0xFF) as u8),
            NodeData::Char(char) => Some(char as u8),
            _ => None
        };

        if byte.is_none() {
            return;
        }

        match self.handle.as_mut() {
            Some(f) => {
                f.write(&[byte.unwrap()])
                    .map_err(|e| println!("{}", e))
                    .unwrap_or(0);
            },
            None => {}
        };
    }

    fn can_open(&self) -> bool {
        self.handle.is_none()
    }

    fn open(&mut self) -> () {
        match File::options().read(true).write(true).open("/dev/ttyS0") {
            Ok(file) => self.handle = Some(file),
            _ => {}
        }
    }

    fn close(&mut self) -> () {
        self.handle = None;
    }
}