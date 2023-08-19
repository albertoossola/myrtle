use alloc::boxed::Box;

use crate::symbols::DataSource;

pub trait HWAdapter {
    fn init(&mut self) -> ();

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn DataSource>;
    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn DataSource>;
    fn set_pwm_pin(&mut self, channel: i32) -> Box<dyn DataSource>;
    fn set_uart(&mut self, tx_pin: i32, rx_pin: i32, baud: i32) -> Box<dyn DataSource>;

    fn set_i2c(&mut self, sda_pin: i32, scl_pin: i32) -> Box<dyn DataSource>;

    fn get_ms_time(&self) -> u64;
    fn get_us_time(&self) -> u64;

    /*fn persist_file(&mut self, name : &str, bytes : &[u8]);
    fn load_file(&mut self, name : &str) -> Option<[u8]>;
    fn delete_file(&mut self, name : &str) -> bool;*/
}
