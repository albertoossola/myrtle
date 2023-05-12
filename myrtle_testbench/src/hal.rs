use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::{stdin, BufRead},
    rc::Rc,
};

use libmyrtle::{DataSource, HWAdapter, NodeData};

pub struct HWState {
    pub pins: BTreeMap<i32, i32>,
}

impl HWState {
    pub fn new() -> HWState {
        HWState {
            pins: BTreeMap::new(),
        }
    }
}

pub struct TestHal {
    pub hw_state: Rc<RefCell<HWState>>,
}

impl TestHal {
    pub fn new() -> TestHal {
        TestHal {
            hw_state: Rc::new(RefCell::new(HWState::new())),
        }
    }
}

impl HWAdapter for TestHal {
    fn init(&mut self) -> () {}

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(PushPull {
            cur_state: 0,
            pin_num,
            state: self.hw_state.clone(),
        });
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(DigitalInput {
            cur_state: 0,
            pin_num,
        });
    }

    fn get_ms_time(&self) -> u64 {
        self.get_ms_time() / 1000
    }

    fn get_us_time(&self) -> u64 {
        todo!()
    }
}

// Push_pull

struct PushPull {
    cur_state: i32,
    pin_num: i32,
    state: Rc<RefCell<HWState>>,
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
            let mut borrow = self.state.borrow_mut();
            borrow.pins.insert(self.pin_num, self.cur_state);

            println!("hal: pin {} set to {}", self.pin_num, self.cur_state);
        }
    }
}

// Digital Input

struct DigitalInput {
    cur_state: i32,
    pin_num: i32,
}

impl DataSource for DigitalInput {
    fn poll(&mut self) -> libmyrtle::NodeData {
        let mut line = String::new();

        //TODO: Read the value from somewhere

        /*_ = stdin().lock().read_line(&mut line).map(|_| {
            let parsed_pin = str::parse::<i32>(line.trim()).unwrap();
            if parsed_pin == self.pin_num {
                self.cur_state = 1 - self.cur_state;
            }
        });*/

        return NodeData::Int(self.cur_state);
    }

    fn can_push(&self) -> bool {
        false
    }

    fn push(&mut self, data: libmyrtle::NodeData) -> () {}
}
