#![no_std]
#![no_main]

extern crate alloc;

extern crate libmyrtle;

mod rp2040_adapter;
mod peripherals;

use core::panic::PanicInfo;

/*
use cortex_m required for the linked
to provide a critical section implementation
*/
use cortex_m as _;
use libmyrtle::{*, DataSource};
use alloc::boxed::Box;
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use libmyrtle::myrtle_instance::MyrtleInstance;
use libmyrtle::shell::datasource_shellio::DataSourceShellIO;
use peripherals::push_pull::PushPullPin;
use rp2040_adapter::RP2040Adapter;
//use libmyrtle::interface::channels::{Channel, MemoryBufferChannel};
//use libmyrtle::myrtle_instance::MyrtleInstance;

#[global_allocator]
static HEAP: Heap = Heap::empty();

/* Main */

pub fn init_heap() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024 * 128;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[link_section = ".boot_loader"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    init_heap();

    let mut hw_adapter = RP2040Adapter::new();

    let serial_port = hw_adapter.set_uart(0, 1, 115200);
    let shell_io = DataSourceShellIO::new(serial_port);

    let mut myrtle_instance =
        MyrtleInstance::new(Box::new(hw_adapter), Box::new(shell_io));

    loop {
        myrtle_instance.step();
    }
}

#[panic_handler]
fn panic(info : &PanicInfo) -> ! {
    let mut hw = RP2040Adapter::new();
    let mut uart = hw.set_uart(0, 1, 115200);

    for c in "panic!".chars() {
        uart.push(NodeData::Char(c));
    }

    let mut led = hw.set_push_pull_pin(25);

    loop {
        led.push(NodeData::Int(1));

        let time = hw.get_ms_time();
        while hw.get_ms_time() - time < 1000 { }
        
        led.push(NodeData::Int(0));
        
        let time = hw.get_ms_time();
        while hw.get_ms_time() - time < 1000 { }
    }

    loop { }
}
