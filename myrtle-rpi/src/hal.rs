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
    pin: i32
}

impl PushPull {
    fn new(pin : i32) -> PushPull {
        std::fs::write("/sys/class/gpio/export", pin.to_string()).unwrap_or(());
        std::fs::write(
            format!("/sys/class/gpio/gpio{}/direction", pin),
            "out"
        ).unwrap();

        PushPull {
            cur_state: 0,
            pin,
        }
    }
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
            std::fs::write(
                format!("/sys/class/gpio/gpio{}/value", self.pin),
                match self.cur_state {0 => "0", _ => "1"}
            ).unwrap();
        }
    }

    fn can_open(&self) -> bool {
        true
    }

    fn open(&mut self) -> () {
    }

    fn close(&mut self) -> () {}
}

// Digital Input

struct DigitalInput {
    cur_state: i32,
    pin: i32,
}

impl DigitalInput {
    pub fn new(pin : i32) -> DigitalInput {
        std::fs::write("/sys/class/gpio/export", pin.to_string()).unwrap_or(());
        std::fs::write(
            format!("/sys/class/gpio/gpio{}/direction", pin),
            "in"
        ).unwrap();

        DigitalInput {
            cur_state: 0,
            pin
        }
    }
}

impl DataSource for DigitalInput {
    fn poll(&mut self) -> NodeData {
        let value: char = std::fs::read(
            format!("/sys/class/gpio/gpio{}/value", self.pin),
        ).ok()
            .and_then(|vec| vec.first().map(|f| *f as char))
            .unwrap_or('_');

        return match value {
            '0' if self.cur_state == 1 => {
                println!("input: 0");
                self.cur_state = 0;
                NodeData::Int(0)
            },
            '1' if self.cur_state == 0 => {
                println!("input: 1");
                self.cur_state = 1;
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
