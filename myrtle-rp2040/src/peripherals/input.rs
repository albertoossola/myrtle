/* Adapter for input gpio */

use libmyrtle::{NodeData, DataSource};
use rp2040_pac as pac;

pub struct InputPin {
    pin: u8,
    last_value: u8
}

impl InputPin {
    pub fn new(pin : usize) -> Self {
        unsafe {
            let reset_bank = 0x4000_c000usize as *const pac::resets::RegisterBlock;
            (*reset_bank).reset.modify(|_, w| w.io_bank0().clear_bit().pads_bank0().clear_bit());

            while (*reset_bank).reset_done.read().io_bank0().bit_is_clear() {}

            /* Init the pin and connect it to SIO */
            let gpio_bank = 0x4001_4000usize as *const pac::io_bank0::RegisterBlock;
            (*gpio_bank).gpio[pin].gpio_ctrl.modify(|_, w| w.funcsel().sio());

            /* Set the pin to input mode */
            let sio_bank = 0xd000_0000usize as *const pac::sio::RegisterBlock;
            (*sio_bank).gpio_oe_clr.write(|reg| reg.bits(1 << pin));
        }

        Self {
            pin: pin as u8,
            last_value: 0
        }
    }
}

impl DataSource for InputPin {
    fn can_push(&self) -> bool {
        true
    }

    fn poll(&mut self) -> NodeData {
        unsafe {
            let sio_bank = 0xd000_0000usize as *const pac::sio::RegisterBlock;
            let input_value = match (*sio_bank).gpio_in.read().gpio_in().bits() & (1 << self.pin) {
                0 => 0,
                _ => 1
            };
        
            if input_value == self.last_value {
                return NodeData::Nil;
            }

            self.last_value = input_value;
            return NodeData::Int(input_value as i32);
        }
    }

    fn push(&mut self, data: NodeData) -> () { }

    fn can_open(&self) -> bool { true }

    fn open(&mut self) -> () { }

    fn close(&mut self) -> () { }
}