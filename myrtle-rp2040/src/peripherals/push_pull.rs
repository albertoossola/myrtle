use libmyrtle::{DataSource, NodeData};
use rp2040_pac as pac;

pub struct PushPullPin {
    pin: usize,
    current_state: u8,
}

impl PushPullPin {
    fn set_value(&mut self, value: i32) {
        let state = value as u8;

        unsafe {
            let sio_bank = 0xd000_0000usize as *const pac::sio::RegisterBlock;

            match state {
                0 => {
                    (*sio_bank).gpio_out_clr.write(|reg| reg.bits(1 << self.pin));
                },
                _ => {
                    (*sio_bank).gpio_out_set.write(|reg| reg.bits(1 << self.pin));
                }
            }
        }

        self.current_state = state;
    }

    pub fn new(pin : usize) -> Self {
        unsafe {
            let reset_bank = 0x4000_c000usize as *const pac::resets::RegisterBlock;
            (*reset_bank).reset.modify(|_, w| w.io_bank0().clear_bit().pads_bank0().clear_bit());

            while (*reset_bank).reset_done.read().io_bank0().bit_is_clear() {}


            let gpio_bank = 0x4001_4000usize as *const pac::io_bank0::RegisterBlock;
            (*gpio_bank).gpio[pin].gpio_ctrl.modify(|_, w| w.funcsel().sio());


            let sio_bank = 0xd000_0000usize as *const pac::sio::RegisterBlock;
            (*sio_bank).gpio_oe_set.write(|reg| reg.bits(1 << pin));
        }


        Self {
            current_state: 0,
            pin
        }
    }
}

impl Drop for PushPullPin {
    fn drop(&mut self) {
        unsafe {
            let bank = 0x4001_4000usize as *const pac::io_bank0::RegisterBlock;
            (*bank).gpio[self.pin].gpio_ctrl.modify(|_, w| w.oeover().normal().funcsel().null());


            let sio_bank = 0xd000_0000usize as *const pac::sio::RegisterBlock;
            (*sio_bank).gpio_oe_clr.write(|reg| reg.bits(1 << self.pin));
        }
    }
}

impl DataSource for PushPullPin {
    fn poll(&mut self) -> NodeData {
        NodeData::Int(self.current_state as i32)
    }

    fn can_push(&self) -> bool {
        true
    }

    fn push(&mut self, data: NodeData) -> () {
       match data {
            NodeData::Int(0) => self.set_value(0),
            NodeData::Int(_) => self.set_value(1),
            NodeData::Bool(true) => self.set_value(1),
            NodeData::Bool(false) => self.set_value(0),
            _ => {}
        };
    }

    fn can_open(&self) -> bool {
        true
    }

    fn open(&mut self) -> () { }

    fn close(&mut self) -> () { }
}
