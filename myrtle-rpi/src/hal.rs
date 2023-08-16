mod push_pull;
mod digital_input;
mod pwm_output;
mod uart;
mod i2c;
mod software_i2c;
mod open_drain;

use push_pull::PushPull;
use digital_input::DigitalInput;
use pwm_output::PwmOutput;
use uart::Uart;
use i2c::I2CAdapter;

use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::{stdin, BufRead},
    rc::Rc,
};

use libmyrtle::{DataSource, HWAdapter, NodeData};

pub struct TestHal { }

impl TestHal {
    pub fn new() -> TestHal {
        TestHal { }
    }
}

impl HWAdapter for TestHal {
    fn init(&mut self) -> () {}

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(PushPull::new(pin_num));
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(DigitalInput::new(pin_num));
    }

    fn set_pwm_pin(&mut self, channel: i32) -> Box<dyn DataSource> {
        return Box::new(PwmOutput::new(channel));
    }

    fn set_uart(&mut self, tx_pin: i32, rx_pin: i32, baud: i32) -> Box<dyn DataSource> {
        return Box::new(Uart::new(tx_pin, rx_pin, baud));
    }

    fn set_i2c(&mut self, sda_pin: i32, scl_pin: i32) -> Box<dyn DataSource> {
        return Box::new(I2CAdapter::new(sda_pin, scl_pin));
    }

    fn get_ms_time(&self) -> u64 {
        self.get_us_time() / 1000
    }

    fn get_us_time(&self) -> u64 {
        todo!()
    }
}