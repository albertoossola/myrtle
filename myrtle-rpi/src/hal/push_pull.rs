use libmyrtle::{DataSource, NodeData};

pub struct PushPull {
    cur_state: i32,
    pin: i32
}

impl PushPull {
    pub fn new(pin : i32) -> PushPull {
        std::fs::write("/sys/class/gpio/export", pin.to_string()).unwrap_or(());
        std::fs::write(
            format!("/sys/class/gpio/gpio{}/direction", pin),
            "out"
        ).unwrap_or(());

        PushPull {
            cur_state: 0,
            pin,
        }
    }
}

impl Drop for PushPull {
    fn drop(&mut self) {
        std::fs::write("/sys/class/gpio/unexport", self.pin.to_string()).unwrap_or(());
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
            ).unwrap_or(());
        }
    }

    fn can_open(&self) -> bool {
        true
    }

    fn open(&mut self) -> () {
        //println!("Opened output pin");
    }

    fn close(&mut self) -> () {
        //println!("Closed output pin");
    }
}
