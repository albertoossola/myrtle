use std::time::Duration;
use libmyrtle::{DataSource, NodeData};
use crate::hal::open_drain::OpenDrain;

pub struct SoftwareI2C {
    pub sda_od : OpenDrain,
    pub scl_od : OpenDrain
}

impl SoftwareI2C {
    fn set_sda(&mut self, value : u8){
        let od_value = match value {
            0 => 0,
            _ => 1
        };

        self.sda_od.push(NodeData::Int(od_value));
    }

    fn set_scl(&mut self, value : u8){
        let od_value = match value {
            0 => 0,
            _ => 1
        };

        self.scl_od.push(NodeData::Int(od_value));
    }

    pub fn delay() {}

    pub fn start(&mut self) {
        self.set_sda(0);
        self.set_scl(0);
    }

    pub fn close(&mut self) {
        self.set_scl(1);
        self.set_sda(1);
    }

    fn tap_scl(&mut self) {
        self.set_scl(1);
        self.set_scl(0);
    }

    pub fn send_byte(&mut self, data : u8){
        //Return SDA to 1 (high impedance)
        self.set_sda(1);

        for i in (0..8).rev() {
            let bit = (data >> i) & 0x01;

            self.set_sda(bit);
            self.tap_scl();
        }

        //Discard ack
        self.tap_scl();
    }

    pub fn read_byte(&mut self) -> u8 {
        //Return SDA to 1 (high impedance)
        self.set_sda(1);

        let mut n : u8 = 0x00;

        for _ in 0..8 {
            self.tap_scl();

            match self.sda_od.poll() {
                NodeData::Int(pin_value) => {
                    n = (n << 1) | (pin_value as u8 & 0x01);
                },
                _ => {
                    n = n << 1;
                }
            };
        }


        //Send ack
        self.set_sda(0);
        self.tap_scl();

        //Return SDA to 1 (high impedance)
        //self.set_sda(1);

        return n;
    }
}