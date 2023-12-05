extern crate alloc;

use alloc::boxed::Box;

use libmyrtle::{DataSource, HWAdapter, NodeData, MemoryDataSource};
use rp2040_pac as pac;

use crate::peripherals::{push_pull::PushPullPin, uart::Uart, input::InputPin};

pub struct RP2040Adapter {
    //pins: BTreeMap<i32, DynPin>
}

impl RP2040Adapter {
    fn set_clock(&self) {
        unsafe {
           let xosc = 0x4002_4000usize as *const pac::xosc::RegisterBlock;
        
            //Set the crystal stabilization wait time
            (*xosc).startup.modify(|_, reg| reg.delay().bits(0xC4));
            
            //Enable the crystal
            (*xosc).ctrl.modify(|_, reg| reg.freq_range()._1_15mhz());
            (*xosc).ctrl.modify(|_, reg| reg.enable().enable());
            
            //Wait for oscillator to stabilize
            let mut xosc_stable = false;
            while !xosc_stable {
                xosc_stable = (*xosc).status.read().stable().bit_is_set();
            }
            
            // From the datasheet:
            // Default PLL configuration:
            //                   REF     FBDIV VCO     POSTDIV
            // PLL SYS: 12 / 1 = 12MHz * 125 = 1500MHz / 6 / 2 = 125MHz
            // ...

            let pll = 0x4002_8000usize as *const pac::pll_sys::RegisterBlock;
            
            // Disable reset bit for pll
            let reset_bank = 0x4000_c000usize as *const pac::resets::RegisterBlock;
            (*reset_bank).reset.modify(|_, w| w.pll_sys().set_bit());
            (*reset_bank).reset.modify(|_, w| w.pll_sys().clear_bit());
            while (*reset_bank).reset_done.read().pll_sys().bit_is_clear() {}

            // The programming sequence for the PLL is as follows:
            // Program the reference clock divider (is a divide by 1 in the RP2040 case)
            (*pll).cs.modify(|_, w| w.refdiv().bits(1));
            
            // Program the feedback divider
            (*pll).fbdiv_int.modify(|_, w| w.fbdiv_int().bits(125));

            // Turn on the main power and VCO
            (*pll).pwr.modify(|_, w| w
                .vcopd().clear_bit()
                .postdivpd().clear_bit()
                .pd().clear_bit()
            );

            // Wait for the VCO to lock (i.e. keep its output frequency stable)
            let mut pll_stable = false;
            while !pll_stable {
                pll_stable = (*pll).cs.read().lock().bit_is_set();
            }

            // Set up post dividers and turn them on
            (*pll).prim.modify(|_, w| w
                .postdiv1().bits(6)
                .postdiv2().bits(2)
            );

            
            let clocks = 0x4000_8000usize as *const pac::clocks::RegisterBlock;
          
            //Set the clk_sys aux source to PLL and switch to it
            (*clocks).clk_sys_ctrl.modify(|_, reg| reg
                .auxsrc().clksrc_pll_sys()
                .src().clksrc_clk_sys_aux()
            );

            //Wait for the transition to complete
            while (*clocks).clk_sys_selected.read().bits() == 0 {}

            //Enable the peripheral clock (for UART and SPI)
            //By default its source is clk_sys
            (*clocks).clk_peri_ctrl.modify(|_, reg| reg.enable().set_bit());

            //Set the reference clock source to xosc, divide by 12 (12Mhz crystal)
            (*clocks).clk_ref_ctrl.modify(|r, w| w.src().xosc_clksrc());
            while (*clocks).clk_ref_selected.read().bits() == 0 { }

            //Set the watchdog divider, required by timers
            let watchdog = 0x4005_8000usize as *const pac::watchdog::RegisterBlock;
            (*watchdog).tick.modify(|_, w| w.bits(12).enable().set_bit()); //Divide clk_ref (xosc) by 12 to get 1Mhz
        }
    }

    fn init_gpio(&self) {}

    pub fn new() -> Self {
        let instance = Self { };

        instance.set_clock();

        return instance;
    }
}

impl HWAdapter for RP2040Adapter {
    fn init(&mut self) -> () {
        
    }

    fn set_push_pull_pin(&mut self, pin_num: i32) -> Box<dyn DataSource> {
        //TODO: Add pin check
        Box::new(PushPullPin::new(pin_num as usize))
    }

    fn set_input_pin(&mut self, pin_num: i32) -> Box<dyn DataSource> {
        Box::new(InputPin::new(pin_num as usize))
    }

    fn set_pwm_pin(&mut self, channel: i32) -> Box<dyn DataSource> {
        todo!()
    }

    fn set_uart(&mut self, tx_pin: i32, rx_pin: i32, baud: i32) -> Box<dyn DataSource> {
        Box::new(Uart::new(baud as u32))
    }

    fn set_i2c(&mut self, sda_pin: i32, scl_pin: i32) -> Box<dyn DataSource> {
        todo!()
    }

    fn get_ms_time(&self) -> u64 {
        self.get_us_time() / 1000
    }

    fn get_us_time(&self) -> u64 {
        let timers = 0x4005_4000usize as *const pac::timer::RegisterBlock;

        unsafe {
            let time_low : u32 = (*timers).timelr.read().bits();
            let time_high : u32 = (*timers).timehr.read().bits();

            let us = ((time_high as u64) << 32) | (time_low as u64);

            return us;
        }
    }
}

/* Data Sources */

