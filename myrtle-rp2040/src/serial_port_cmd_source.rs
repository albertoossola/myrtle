use libmyrtle::interface::command_sources::{Command, CommandSource};
use rp2040_hal::{Sio, uart::{}};
use rp2040_hal::gpio::{Clock, Pins, Uart};
use rp2040_hal::uart::{DataBits, StopBits, UartConfig, UartPeripheral};
use rp2040_hal::clocks;
use rp2040_hal::clocks::init_clocks_and_plls;

pub struct SerialPortCommandSource {

}

impl CommandSource for SerialPortCommandSource {
    fn poll(&mut self) -> Option<Command> {
        let mut peripherals = rp2040_hal::pac::Peripherals::take().unwrap();
        let sio = Sio::new(peripherals.SIO);
        let pins = Pins::new(peripherals.IO_BANK0, peripherals.PADS_BANK0, sio.gpio_bank0, &mut peripherals.RESETS);
        let mut clocks = init_clocks_and_plls(XOSC_CRYSTAL_FREQ, peripherals.XOSC, peripherals.CLOCKS, peripherals.PLL_SYS, peripherals.PLL_USB, &mut peripherals.RESETS, &mut watchdog).ok().unwrap();


        let pins = (
            pins.gpio0.into_function(),
            pins.gpio1.into_function(),
        );

        let uart = UartPeripheral::new(peripherals.UART0, pins, &mut peripherals.RESETS)
            .enable(
                UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
                p
            ).unwrap();
    }
}

impl SerialPortCommandSource {
    pub fn new() -> Self {
        return SerialPortCommandSource {
        };
    }
}