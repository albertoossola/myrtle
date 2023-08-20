use std::time::Duration;
use libmyrtle::{DataSource, NodeData};
use crate::hal::open_drain::OpenDrain;

pub struct SoftwareI2C {
    pub sda_od : OpenDrain,
    pub scl_od : OpenDrain,
    pub ack_to_send : bool
}

impl SoftwareI2C {
    fn set_sda(&mut self, value : u8){
        let od_value = match value {
            0 => 0,
            _ => 1
        };

        self.sda_od.push(NodeData::Int(od_value));

        std::thread::sleep(Duration::from_micros(10));
    }

    fn set_scl(&mut self, value : u8){
        let od_value = match value {
            0 => 0,
            _ => 1
        };

        self.scl_od.push(NodeData::Int(od_value));

        std::thread::sleep(Duration::from_micros(10));
    }

    pub fn delay() {}

    pub fn start(&mut self) {
        self.ack_to_send = false;

        self.set_sda(1);
        self.set_scl(1);

        self.set_sda(0);
        self.set_scl(0);
    }

    pub fn close(&mut self) {
        if self.ack_to_send {
            self.send_nack();
            self.ack_to_send = false;
        }

        self.set_scl(0);
        self.set_sda(0);

        self.set_scl(1);
        self.set_sda(1);
    }

    fn tap_scl(&mut self) {
        self.set_scl(1);
        self.set_scl(0);
    }

    fn send_ack(&mut self) {
        self.set_scl(0);

        self.set_sda(0);
        self.tap_scl();
    }

    fn send_nack(&mut self) {
        self.set_scl(0);

        self.set_sda(1);
        self.tap_scl();
    }

    pub fn send_byte(&mut self, data : u8){
        if self.ack_to_send {
            self.send_ack();
            self.ack_to_send = false;
        }

        self.set_sda(0);
        self.set_scl(0);

        for i in (0..8).rev() {
            let bit = (data >> i) & 0x01;

            self.set_sda(bit);
            self.tap_scl();
        }

        // Get ACK
        // Force SCL low
        self.set_scl(0);

        // Release SDA
        self.set_sda(1);

        // Release SCL
        self.set_scl(1);

        // Keep SCL low between bytes
        self.set_scl(0);
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.ack_to_send {
            self.send_ack();
            self.ack_to_send = false;
        }

        self.set_sda(1);
        self.set_scl(0);

        let mut n : u8 = 0x00;

        for i in 0..8 {
            self.set_scl(1);

            match self.sda_od.poll() {
                NodeData::Int(pin_value) => {
                    n = (n << 1) | (pin_value as u8 & 0x01);
                },
                _ => {
                    n = n << 1;
                }
            };

            self.set_scl(0);

        }

        self.ack_to_send = true;

        return n;
    }
}