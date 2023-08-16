use std::thread::sleep;
use std::time::Duration;
use libmyrtle::{DataSource, NodeData};

pub struct OpenDrain {
    cur_state: i32,
    pin: i32
}

impl OpenDrain {
    pub fn new(pin : i32) -> OpenDrain {
        _ = std::fs::write("/sys/class/gpio/export", pin.to_string());

        sleep(Duration::from_millis(50));

        _ = std::fs::write(
            format!("/sys/class/gpio/gpio{}/direction", pin),
            "out"
        ).map_err(|e| {println!("{}", e)});

        _ = std::fs::write(
            format!("/sys/class/gpio/gpio{}/active_low", pin),
            "1"
        ).map_err(|e| {println!("{}", e)});

        OpenDrain {
            cur_state: 0,
            pin,
        }
    }
}

impl Drop for OpenDrain {
    fn drop(&mut self) {
        std::fs::write("/sys/class/gpio/unexport", self.pin.to_string()).unwrap_or(());
    }
}

impl DataSource for OpenDrain {
    fn poll(&mut self) -> libmyrtle::NodeData {
        let path = format!("/sys/class/gpio/gpio{}/value", self.pin);
        let str_value = std::fs::read_to_string(path).unwrap();

        return NodeData::Int(match str_value.as_str() { "1" => 1, _ => 0 });
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

        //It's open drain, so a 0 actually corresponds to an active state
        if last_state != self.cur_state {
            std::fs::write(
                format!("/sys/class/gpio/gpio{}/value", self.pin),
                match self.cur_state {0 => "1", _ => "0"}
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
