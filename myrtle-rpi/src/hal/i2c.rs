use libmyrtle::{DataSource, NodeData};
use i2cdev::{linux::LinuxI2CDevice, core::I2CDevice};
use crate::hal::open_drain::OpenDrain;
use crate::hal::software_i2c::SoftwareI2C;

enum I2CStatus {
    Idle,
    WaitingAddress,
    WaitingData,
}

pub struct I2CAdapter {
    status: I2CStatus,
    i2c_handle: SoftwareI2C,
    last_value_on_bus: Option<u8>
}

impl DataSource for I2CAdapter {
    fn poll(&mut self) -> NodeData {
        match self.last_value_on_bus {
            None => NodeData::Nil,
            Some(n) => NodeData::Int(n as i32)
        }
    }

    fn can_push(&self) -> bool {
        //If it's to be opened or waiting for data
        return true;
    }

    fn push(&mut self, data: NodeData) -> () {
        match self.status {
            I2CStatus::WaitingAddress => match data {
                NodeData::Int(n) => {
                    self.i2c_handle.start();
                    self.i2c_handle.send_byte(n as u8);

                    println!("Set slave address to: {}", n);
                    self.status = I2CStatus::WaitingData;
                },
                _ => {}
            },
            I2CStatus::WaitingData => match data {
                NodeData::Int(n) => {
                    self.i2c_handle.send_byte(n as u8);
                    self.last_value_on_bus = Some(n as u8);

                    println!("I2C - Adding byte: {}", n);
                },
                NodeData::Nil => {
                    let read_data = 0x00;
                    self.last_value_on_bus = Some(read_data);
                },
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
        self.last_value_on_bus = None;

        match self.status {
            I2CStatus::Idle => {
                self.status = I2CStatus::WaitingAddress;
            }
            _ => {}
        }
    }

    fn close(&mut self) -> () {
        //self.i2c_handle.smbus_write_byte(&self.write_buffer).ok();

        self.i2c_handle.close();
        self.last_value_on_bus = None;
        self.status = I2CStatus::Idle;

        println!("I2C - data sent");
    }
}

impl I2CAdapter {
    pub fn new(sda_pin: i32, scl_pin: i32) -> Self {
        if sda_pin != 2 || scl_pin != 3 {
            panic!("Can't open I2C device on selected pins");
        }

        return I2CAdapter {
            i2c_handle: SoftwareI2C {
                sda_od: OpenDrain::new(2),
                scl_od: OpenDrain::new(3)
            },
            last_value_on_bus: None,
            status: I2CStatus::Idle,
        };
    }
}
