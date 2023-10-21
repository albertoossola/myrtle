#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

extern crate libmyrtle;

mod rpico;
mod serial_port_cmd_source;

/*
use cortex_m required for the linked
to provide a critical section implementation
*/
use cortex_m as _;

use libmyrtle::*;

use rpico::*;

use crate::serial_port_cmd_source::SerialPortCommandSource;
use alloc::vec::Vec;
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use libmyrtle::interface::channels::{Channel, MemoryBufferChannel};
use libmyrtle::myrtle_instance::MyrtleInstance;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

/* Main */

pub fn init_heap(heap_buffer: &[u8]) {
    let heap_start = heap_buffer.as_ptr().cast_mut() as usize;
    //let heap_end = ;
    let heap_size = heap_buffer.len();
    unsafe {
        ALLOCATOR.init(heap_start, heap_size);
    }
}

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let heap_space = [0_u8; 128 * 1024];
    init_heap(&heap_space);

    let hw_adapter = RPicoAdapter::init();
    let command_source = SerialPortCommandSource::new();

    let channels: Vec<Box<dyn Channel>> = vec![Box::new(MemoryBufferChannel::new())];

    let mut myrtle_instance =
        MyrtleInstance::new(Box::new(hw_adapter), Box::new(command_source), channels);

    loop {
        myrtle_instance.step();
    }
}
