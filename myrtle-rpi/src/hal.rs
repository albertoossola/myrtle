use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::{stdin, BufRead},
    rc::Rc,
};

use rppal::gpio::{Gpio, OutputPin, Pin, InputPin, Level};
use libmyrtle::{DataSource, HWAdapter, NodeData};

pub struct TestHal {
    pub gpio : Gpio
}

impl TestHal {
    pub fn new() -> TestHal {
        TestHal {
            gpio: Gpio::new().unwrap()
        }
    }
}

impl HWAdapter for TestHal {
    fn init(&mut self) -> () {}

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {

        return Box::new(PushPull {
            cur_state: 0,
            pin: self.gpio.get(pin_num as u8).unwrap().into_output()
        });
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(DigitalInput {
            cur_state: Level::High,
            pin: self.gpio.get(pin_num as u8).unwrap().into_input_pullup(),
        });
    }

    fn get_ms_time(&self) -> u64 {
        self.get_us_time() / 1000
    }

    fn get_us_time(&self) -> u64 {
        todo!()
    }
}

// Push_pull

struct PushPull {
    cur_state: i32,
    pin: OutputPin
}

impl DataSource for PushPull {
    fn poll(&mut self) -> libmyrtle::NodeData {
        return NodeData::Int(self.cur_state);
    }

    fn can_push(&self) -> bool {
        true
    }

    fn push(&mut self, data: libmyrtle::NodeData) -> () {
        let last_state = self.cur_state;

        match data {
            NodeData::Int(0) => self.cur_state = 0,
            NodeData::Int(_) => self.cur_state = 1,
            _ => {}
        }

        if last_state != self.cur_state {
            match self.cur_state {
                0 => self.pin.set_low(),
                _ => self.pin.set_high()
            }

            //println!("hal: pin {} set to {}", self.pin.pin(), self.cur_state);
        }
    }

    fn can_open(&self) -> bool {
        true
    }

    fn open(&mut self) -> () {}

    fn close(&mut self) -> () {}
}

// Digital Input

struct DigitalInput {
    cur_state: Level,
    pin: InputPin,
}

impl DataSource for DigitalInput {
    fn poll(&mut self) -> NodeData {
        match self.pin.read() {
            Level::Low if self.cur_state == Level::High => {
                self.cur_state = Level::Low;
                NodeData::Int(0)
            },
            Level::High if self.cur_state == Level::Low => {
                self.cur_state = Level::High;
                NodeData::Int(1)
            },
            _ => NodeData::Nil
        }
    }

    fn can_push(&self) -> bool {
        false
    }

    fn push(&mut self, data: libmyrtle::NodeData) -> () {}

    fn can_open(&self) -> bool {
        true
    }

    fn open(&mut self) -> () {}

    fn close(&mut self) -> () {}
}
