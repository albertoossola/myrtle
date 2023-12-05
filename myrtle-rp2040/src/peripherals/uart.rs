use libmyrtle::{DataSource, NodeData};
use rp2040_pac as pac;

pub struct Uart;

impl DataSource for Uart {
    fn poll(&mut self) -> libmyrtle::NodeData {
        let uart_reg_block = 0x4003_4000usize as *const pac::uart0::RegisterBlock;

        unsafe {
            let has_data = (*uart_reg_block).uartfr.read().rxfe().bit_is_clear();

            if !has_data {
                return NodeData::Nil;
            }

            let data = (*uart_reg_block).uartdr.read().data().bits();

            return NodeData::Int(data as i32);
        }
    }

    fn can_push(&self) -> bool {
        let uart_reg_block = 0x4003_4000usize as *const pac::uart0::RegisterBlock;

        unsafe {
            let fifo_free = (*uart_reg_block).uartfr.read().txff().bit_is_clear();
            return fifo_free;
        }
    }

    fn push(&mut self, data: libmyrtle::NodeData) -> () {
        let uart_reg_block = 0x4003_4000usize as *const pac::uart0::RegisterBlock;

        let byte_or_none = match data {
            NodeData::Int(i) => Some(i as u8),
            NodeData::Char(c) => Some(c as u8),
            _ => None
        };

        if let Some(byte) = byte_or_none {
            unsafe {
                (*uart_reg_block).uartdr.write(|w| w.data().bits(byte));
            }
        }
    }

    fn can_open(&self) -> bool { true }

    fn open(&mut self) -> () { }

    fn close(&mut self) -> () { }
}

impl Uart {
    pub fn new(baud_rate : u32) -> Self {
        let uart_reg_block = 0x4003_4000usize as *const pac::uart0::RegisterBlock;
        let io_reg_block = 0x4001_4000usize as *const pac::io_bank0::RegisterBlock;

        unsafe {
            //Setup resets
            let reset_bank = 0x4000_c000usize as *const pac::resets::RegisterBlock;
            (*reset_bank).reset.modify(|_, w| w.uart0().set_bit());
            (*reset_bank).reset.modify(|_, w| w.io_bank0().clear_bit().pads_bank0().clear_bit().uart0().clear_bit());

            while (*reset_bank).reset_done.read().uart0().bit_is_clear() {}

            Self::set_baud_rate(125_000_000, baud_rate);

            (*io_reg_block).gpio[0].gpio_ctrl.modify(|_, w| w.funcsel().uart()); //Set pin 0 to UART mode
            (*io_reg_block).gpio[1].gpio_ctrl.modify(|_, w| w.funcsel().uart()); //Set pin 1 to UART mode

            (*uart_reg_block).uartlcr_h.modify(|_, cr| cr.fen().set_bit().wlen().bits(0b11)); //Enable FIFO and set length to 8 bit
            (*uart_reg_block).uartcr.modify(|_, cr| cr.uarten().set_bit().txe().set_bit().rxe().set_bit()); //Enable the UART peripheral, RX and TX

            (*uart_reg_block).uartdmacr.modify(|_, cr| cr.txdmae().set_bit().rxdmae().set_bit()); //Enable DMA for RX and TX
        }

        Uart
    }

    fn set_baud_rate(clock_speed : u32, baud_rate : u32) {
        let baud_rate_div : u32 = (8 * clock_speed) / baud_rate;
        let mut baud_ibrd : u32 = baud_rate_div >> 7;
        let baud_fbrd;
        
        if baud_ibrd == 0 {
            baud_ibrd = 1;
            baud_fbrd = 0;
        } else if baud_ibrd >= 65535 {
            baud_ibrd = 65535;
            baud_fbrd = 0;
        }
        else {
            baud_fbrd = ((baud_rate_div & 0x7f) + 1) / 2;
        }

        unsafe {
            let uart_reg_block = 0x4003_4000usize as *const pac::uart0::RegisterBlock;
            (*uart_reg_block).uartibrd.modify(|_, reg| reg.baud_divint().bits(baud_ibrd as u16));
            (*uart_reg_block).uartfbrd.modify(|_, reg| reg.baud_divfrac().bits(baud_fbrd as u8));

            //Dummy write to line register - see manual
            (*uart_reg_block).uartlcr_h.modify(|_, reg| reg.bits(0));
        }
    }
}