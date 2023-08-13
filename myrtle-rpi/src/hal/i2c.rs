use crate::hal::i2c::I2CStatus::Idle;
use libmyrtle::{DataSource, NodeData};
use std::borrow::Cow::Owned;
use std::ffi::c_int;
use std::fs::{read, File, OpenOptions};
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd, RawFd};
use i2cdev::core::{I2CDevice, I2CTransfer};
use i2cdev::linux::LinuxI2CDevice;

enum I2CStatus {
    Idle,
    WaitingAddress,
    WaitingData,
}

pub struct I2CAdapter {
    status: I2CStatus,
    i2c_handle: LinuxI2CDevice,
    write_buffer: Vec<u8>
}

impl DataSource for I2CAdapter {
    fn poll(&mut self) -> NodeData {
        return NodeData::Nil;
    }

    fn can_push(&self) -> bool {
        //If it's to be opened or waiting for data
        return true;
    }

    fn push(&mut self, data: NodeData) -> () {
        match self.status {
            I2CStatus::WaitingAddress => match data {
                NodeData::Int(n) => {
                    self.i2c_handle.set_slave_address(n as u16).unwrap();

                    println!("Set slave address to: {}", n);

                    self.status = I2CStatus::WaitingData;
                },
                _ => {}
            },
            I2CStatus::WaitingData => match data {
                NodeData::Int(n) => {
                    self.write_buffer.push(n as u8);

                    println!("I2C - Adding byte: {}", n);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn can_open(&self) -> bool {
        return match self.status {
            I2CStatus::Idle => true,
            _ => false,
        };
    }

    fn open(&mut self) -> () {
        self.write_buffer.clear();
        match self.status {
            I2CStatus::Idle => {
                self.status = I2CStatus::WaitingAddress;
            }
            _ => {}
        }
    }

    fn close(&mut self) -> () {
        self.i2c_handle.write(&self.write_buffer).ok();

        self.status = I2CStatus::Idle;
        self.write_buffer.clear();

        println!("I2C - data sent");
    }
}

impl I2CAdapter {
    pub fn new(sda_pin: i32, scl_pin: i32) -> Self {
        if sda_pin != 2 || scl_pin != 3 {
            panic!("Can't open I2C device on selected pins");
        }

        return I2CAdapter {
            i2c_handle: LinuxI2CDevice::new("/dev/i2c-1", 0x00).unwrap(),
            write_buffer: vec![],
            status: I2CStatus::Idle,
        };
    }
}
