use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use libmyrtle::{DataSource, NodeData};
use rppal::gpio::{Pin, Gpio, InputPin, OutputPin, Level};

pub struct OpenDrain {
    cur_state: i32,
    pin_num: i32
}

impl OpenDrain {
    pub fn new(pin_num: i32) -> OpenDrain {
        let pin = Gpio::new().unwrap().get(pin_num as u8).unwrap();

        OpenDrain {
            cur_state: 0,
            pin_num
        }
    }
}

impl Drop for OpenDrain {
    fn drop(&mut self) {
        println!("Closing open-drain pin {}", self.pin_num);

        let pin = Gpio::new().unwrap().get(self.pin_num as u8).unwrap();
        pin.into_input_pullup();

        //let mut pin = self.pin.take().unwrap();


        //let s = Gpio::new().unwrap().get(self.pin_num as u8).unwrap();
        //self.pin = Some(pin);

        //std::fs::write("/sys/class/gpio/unexport", self.pin_num.to_string()).unwrap();
    }
}

impl DataSource for OpenDrain {
    fn poll(&mut self) -> libmyrtle::NodeData {
        let pin = Gpio::new().unwrap().get(self.pin_num as u8).unwrap();
        let level = pin.read();

        return match level {
            Level::High => NodeData::Int(1),
            Level::Low => NodeData::Int(0)
        };

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

                //Set to drain
                let pin = Gpio::new().unwrap().get(self.pin_num as u8).unwrap();
                let mut output_pin = pin.into_output_low();
                output_pin.set_reset_on_drop(false);
                //self.pin = Some(pin);
            },
            NodeData::Int(_) => {
                self.cur_state = 1;

                //Set to open
                let pin = Gpio::new().unwrap().get(self.pin_num as u8).unwrap();
                let mut input_pin = pin.into_input_pullup();
                input_pin.set_reset_on_drop(false);
                //self.pin = Some(pin);
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
