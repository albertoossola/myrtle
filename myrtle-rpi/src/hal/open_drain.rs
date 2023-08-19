use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use libmyrtle::{DataSource, NodeData};
use gpio_cdev::{Chip, Line, LineHandle, LineRequestFlags};

pub struct OpenDrain {
    cur_state: i32,
    pin_num: i32,
    gpio_line: LineHandle
}

impl OpenDrain {
    pub fn new(pin_num: i32) -> OpenDrain {

        let line = Chip::new("/dev/gpiochip0")
            .unwrap()
            .get_line(pin_num as u32)
            .unwrap();

        let line_handle = line.request(
            LineRequestFlags::OPEN_DRAIN | LineRequestFlags::OUTPUT,
            1,
            &format!("pin{}", pin_num)
        ).unwrap();

        OpenDrain {
            cur_state: 0,
            pin_num,
            gpio_line: line_handle
        }
    }
}

impl Drop for OpenDrain {
    fn drop(&mut self) {
        println!("Closing open-drain pin {}", self.pin_num);


        self.gpio_line
            .line()
            .request(LineRequestFlags::INPUT, 0, "");

        //let mut pin = self.pin.take().unwrap();


        //let s = Gpio::new().unwrap().get(self.pin_num as u8).unwrap();
        //self.pin = Some(pin);

        //std::fs::write("/sys/class/gpio/unexport", self.pin_num.to_string()).unwrap();
    }
}

impl DataSource for OpenDrain {
    fn poll(&mut self) -> libmyrtle::NodeData {
        let level = self.gpio_line.get_value().unwrap();

        NodeData::Int(level as i32)

        /*let path = format!("/sys/class/gpio/gpio{}/value", self.pin_num);
        let str_value = std::fs::read_to_string(path).unwrap();

        return NodeData::Int(match str_value.as_str() { "1" => 1, _ => 0 });*/
    }

    fn can_push(&self) -> bool {
        true
    }

    fn push(&mut self, data: libmyrtle::NodeData) -> () {
        let last_state = self.cur_state;

        match data {
            NodeData::Int(0) => {
                self.cur_state = 0;
                self.gpio_line.set_value(0);
            },
            NodeData::Int(_) => {
                self.cur_state = 1;
                self.gpio_line.set_value(1);
            },
            _ => {}
        }

        /*if last_state != self.cur_state {
            std::fs::write(
                format!("/sys/class/gpio/gpio{}/value", self.pin_num),
                match self.cur_state {0 => "1", _ => "0"}
            ).unwrap_or(());
        }*/
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
