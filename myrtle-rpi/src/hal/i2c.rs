use libmyrtle::{DataSource, NodeData};
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
        match self.last_value_on_bus.take() {
            None => NodeData::Nil,
            Some(n) => {
                NodeData::Int(n as i32)
            }
        }
    }

    fn can_push(&self) -> bool {
        self.last_value_on_bus.is_none()
    }

    fn push(&mut self, data: NodeData) -> () {
        match self.status {
            I2CStatus::WaitingAddress => match data {
                NodeData::Int(n) => {
                    self.i2c_handle.start();
                    self.i2c_handle.send_byte(n as u8);

                    self.status = I2CStatus::WaitingData;
                },
                _ => {}
            },
            I2CStatus::WaitingData => match data {
                NodeData::Int(n) => {
                    self.i2c_handle.send_byte(n as u8);
                    //self.last_value_on_bus = Some(n as u8);
                },
                NodeData::Blank => {
                    let read_data = self.i2c_handle.read_byte();
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
        self.i2c_handle.close();
        self.last_value_on_bus = None;
        self.status = I2CStatus::Idle;
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
                scl_od: OpenDrain::new(3),
                ack_to_send: false
            },
            last_value_on_bus: None,
            status: I2CStatus::Idle,
        };
    }
}
