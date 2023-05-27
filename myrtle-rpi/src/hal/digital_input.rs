use libmyrtle::{DataSource, NodeData};

pub struct DigitalInput {
    cur_state: i32,
    pin: i32,
}

impl DigitalInput {
    pub fn new(pin : i32) -> DigitalInput {
        std::fs::write("/sys/class/gpio/export", pin.to_string()).unwrap_or(());
        std::fs::write(
            format!("/sys/class/gpio/gpio{}/direction", pin),
            "in"
        ).unwrap_or(());

        DigitalInput {
            cur_state: 0,
            pin
        }
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        std::fs::write("/sys/class/gpio/unexport", self.pin.to_string()).unwrap_or(());
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
                //println!("input: 0");
                self.cur_state = 0;
                NodeData::Int(0)
            },
            '1' if self.cur_state == 0 => {
                //println!("input: 1");
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
