use libmyrtle::{DataSource, HWAdapter, NodeData};

pub struct TestHal {}

impl HWAdapter for TestHal {
    fn init() -> Self {
        return TestHal {};
    }

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(PushPull {
            cur_state: 0,
            pin_num,
        });
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        todo!()
    }

    fn get_ms_time(&self) -> u64 {
        todo!()
    }
}

// Push_pull

struct PushPull {
    cur_state: i32,
    pin_num: i32,
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
            println!("hal: pin {} set to {}", self.pin_num, self.cur_state);
        }
    }
}
