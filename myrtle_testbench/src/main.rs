extern crate liblink;
extern crate libmyrtle;

use std::{
    alloc::System,
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, UdpSocket},
    time::{SystemTime, UNIX_EPOCH},
};

use liblink::*;
use libmyrtle::*;

mod hal;
pub use hal::TestHal;
use ringbuffer::{
    ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferRead, RingBufferWrite,
};

fn main() {
    println!("Myrtle Testbench - v0.1.0");

    println!("Initializing HAL...");

    let mut hal_instance = hal::TestHal {};
    hal_instance.init();

    /* println!("Loading program from file...");

    let source: String = String::from_utf8(fs::read("./main.myr").unwrap()).unwrap();

    let mut machine_ast = parse_program(source.as_str()).unwrap().1;

    let mut machine = make_program(&mut hal_instance, &mut machine_ast).unwrap();

    println!("Loaded, running.");*/

    let mut source_buffer: [u8; 4096] = [0x00; 4096];
    let mut source_buffer_index: usize = 0;

    println!("Opening serial socket..");

    let listener: UdpSocket = UdpSocket::bind("0.0.0.0:42069").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut sender: Option<SocketAddr> = None;

    let mut machine = Machine::make_blank();

    println!("Listening for UDP packets");

    {
        let mut transceiver = RTx::new();
        let mut received_frame: [u8; MAX_DATA] = [0 as u8; MAX_DATA];

        let mut rx_buf: liblink::Buffer = liblink::Buffer::new();
        let mut tx_buf: liblink::Buffer = liblink::Buffer::new();

        loop {
            if rx_buf.can_write() {
                let mut buf = [0; 64];
                listener
                    .recv_from(&mut buf)
                    .and_then(|(l, addr)| match l {
                        0 => Ok(l),
                        _ => {
                            sender = Some(addr);

                            buf.iter().take(l).for_each(|b| rx_buf.write(*b));
                            Ok(l)
                        }
                    })
                    .unwrap_or(0);
            }

            tx_buf
                .read()
                .map(|txdata| sender.map(|addr| listener.send_to(&[txdata], addr)));

            let rec = transceiver.update(&mut tx_buf, &mut rx_buf, &mut received_frame);

            rec.map(|l| {
                core::str::from_utf8(&received_frame[..l])
                    .map(|rec_str| match rec_str {
                        "@@clear" => {
                            println!("LOG > received clear command");
                            source_buffer.fill(0x00);
                            source_buffer_index = 0;
                        }
                        "@@run" => {
                            println!("LOG > received run command");

                            //TODO: Do this without result -> option conversion
                            let mut machine_ast = core::str::from_utf8(&source_buffer)
                                .ok()
                                .and_then(|str| parse_program(str).ok())
                                .and_then(|(_, mut ast)| {
                                    make_program(&mut hal_instance, &mut ast).ok()
                                })
                                .map(|m| machine = m);
                        }
                        _ => {
                            println!("LOG > {}", rec_str);

                            for byte in received_frame {
                                source_buffer[source_buffer_index] = byte;
                                source_buffer_index += 1;

                                if source_buffer_index >= source_buffer.len() {
                                    break;
                                }
                            }
                        }
                    })
                    .unwrap_or(());

                transceiver.poll_next();
            });

            let context = MachineRunContext {
                current_ticks: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            };

            machine.run(context)
        }
    }
}
