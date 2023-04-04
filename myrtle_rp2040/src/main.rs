#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

extern crate libmyrtle;

mod rpico;

/*
use cortex_m required for the linked
to provide a critical section implementation
*/
use cortex_m as _;

use libmyrtle::*;

use rpico::*;

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec};
use cortex_m_rt::entry;
use embedded_alloc::Heap;

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

    //Initialize hardware
    let mut adapter = rpico::RPicoAdapter::init();

    let led_pin_data_source = adapter.set_push_pull_pin(25);
    let led_pin_symbol = Symbol::new(led_pin_data_source);

    let mut variables = BTreeMap::new();

    variables.insert(String::from("led"), led_pin_symbol);

    let set_var_node = Node {
        behaviour: Box::new(SetVarBehaviour::new(String::from("led"))),
        in_buf: NodeBuffer {
            data: NodeData::Nil,
        },
        next: None,
    };

    let emit_node = Node {
        behaviour: Box::new(EmitBehaviour::new(vec![
            NodeData::Int(1),
            NodeData::Int(0),
            NodeData::Int(1),
            NodeData::Int(0),
            NodeData::Int(0),
            NodeData::Int(0),
            NodeData::Int(0),
            NodeData::Int(0),
            NodeData::Int(0),
            NodeData::Int(0),
            NodeData::Int(0),
        ])),
        in_buf: NodeBuffer {
            data: NodeData::Nil,
        },
        next: Some(Box::new(set_var_node)),
    };

    let timer_node = Node {
        behaviour: Box::new(TimerBehaviour::new(100)),
        in_buf: NodeBuffer {
            data: NodeData::Nil,
        },
        next: Some(Box::new(emit_node)),
    };

    let mut machine = Machine {
        cur_state: String::from("myrtle_entry"),
        variables: variables,
        states: BTreeMap::from([(
            String::from("myrtle_entry"),
            State {
                vars: BTreeMap::new(),
                flows: vec![timer_node],
            },
        )]),
    };

    loop {
        let context = MachineRunContext {
            current_ticks: adapter.get_ms_time(),
        };

        machine.run(context);
    }
}
