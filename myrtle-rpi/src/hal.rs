mod digital_input;
mod i2c;
mod open_drain;
mod push_pull;
mod pwm_output;
mod software_i2c;
mod uart;

use digital_input::DigitalInput;
use i2c::I2CAdapter;
use push_pull::PushPull;
use pwm_output::PwmOutput;
use uart::Uart;





use libmyrtle::{DataSource, HWAdapter};

pub struct RaspberryPiHal {}

impl RaspberryPiHal {
    pub fn new() -> RaspberryPiHal {
        RaspberryPiHal {}
    }
}

impl HWAdapter for RaspberryPiHal {
    fn init(&mut self) -> () {}

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(PushPull::new(pin_num));
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn libmyrtle::DataSource> {
        return Box::new(DigitalInput::new(pin_num));
    }

    fn set_pwm_pin(&mut self, channel: i32) -> Box<dyn DataSource> {

        return Box::new(PwmOutput::new(channel));
    }

    fn set_uart(&mut self, tx_pin: i32, rx_pin: i32, baud: i32) -> Box<dyn DataSource> {
        return Box::new(Uart::new(tx_pin, rx_pin, baud));
    }

    fn set_i2c(&mut self, sda_pin: i32, scl_pin: i32) -> Box<dyn DataSource> {
        return Box::new(I2CAdapter::new(sda_pin, scl_pin));
    }

    fn get_ms_time(&self) -> u64 {
        self.get_us_time() / 1000
    }

    fn get_us_time(&self) -> u64 {
        todo!()
    }
}
