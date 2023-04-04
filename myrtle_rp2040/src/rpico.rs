extern crate alloc;
use core::fmt::Error;

use alloc::{boxed::Box, collections::BTreeMap};

use embedded_hal::digital::v2::OutputPin;
use libmyrtle::{DataSource, HWAdapter, NodeData};
use rp2040_hal::{
    clocks::init_clocks_and_plls,
    gpio::{self, DynPin, PinState, Uart},
    pac,
    timer::Instant,
    Sio, Timer, Watchdog,
};

pub struct RPicoAdapter {
    pins: BTreeMap<i32, DynPin>,
    timer: Timer,
}

impl RPicoAdapter {
    fn set_clock(&self) {}

    fn init_gpio(&self) {}
}

impl HWAdapter for RPicoAdapter {
    fn init() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

        // External high-speed crystal on the pico board is 12Mhz
        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let pins = gpio::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let mut pin_map = BTreeMap::new();

        pin_map.insert(0, pins.gpio0.into());
        pin_map.insert(1, pins.gpio1.into());
        pin_map.insert(2, pins.gpio2.into());
        pin_map.insert(3, pins.gpio3.into());
        pin_map.insert(25, pins.gpio25.into());

        return RPicoAdapter {
            pins: pin_map,
            timer: Timer::new(pac.TIMER, &mut pac.RESETS),
        };
    }

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn DataSource> {
        let mut pin = self.pins.remove(&pin_num).unwrap();
        pin.into_push_pull_output();

        Box::new(OutputPinDataSource {
            pin,
            current_state: PinState::Low,
        })
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn DataSource> {
        todo!()
    }

    fn get_ms_time(&self) -> u64 {
        self.timer.get_counter().ticks() / 1000
    }
}

/* Data Sources */

struct OutputPinDataSource {
    pin: DynPin,
    current_state: PinState,
}

impl OutputPinDataSource {
    fn set_value(&mut self, value: i32) {
        let state = match value {
            0 => gpio::PinState::Low,
            _ => gpio::PinState::High,
        };

        self.current_state = state;

        self.pin.set_state(state).unwrap();
    }
}

impl DataSource for OutputPinDataSource {
    fn poll(&mut self) -> NodeData {
        let state_int = match self.current_state {
            PinState::High => 1,
            PinState::Low => 0,
        };

        NodeData::Int(state_int)
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
}
