use std::fmt::Write;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};

use liblink::{Buffer, RTx, MAX_DATA};

pub struct Listener {
    pub rtx: RTx,
    tx_buf: liblink::Buffer,
    rx_buf: liblink::Buffer,

    socket: TcpListener,
    stream: Option<TcpStream>,
    source_buffer: String,
}

impl Listener {
    pub fn init(&mut self) {
        println!("Opening TCP socket..");
        self.socket.set_nonblocking(true).unwrap();
        self.stream = None;

        println!("Listening for TCP packets");
    }

    pub fn update(&mut self) -> Option<String> {
        match self.stream.as_mut() {
            Some(stream) => {
                let mut buf = [0; 64];

                _ = stream
                    .read(&mut buf)
                    .map(|len|
                        buf.iter()
                            .take(len)
                            .for_each(|b| self.source_buffer.push(*b as char))
                    );
            },
            None => {
                match self.socket.accept() {
                    Ok((stream, _)) => self.stream = Some(stream),
                    _ => {}
                };
            }
        }

        if self.source_buffer.starts_with("@@clear") {
            println!("Receiving program...");

            self.source_buffer = self.source_buffer
                .strip_prefix("@@clear")
                .unwrap_or("")
                .to_string();

            None
        }
        else if self.source_buffer.ends_with("@@run") {
            println!("Program received");

            for _ in 0.."@@run".len() {
                self.source_buffer.pop();
            }

            let ret_val = Some(self.source_buffer.clone());

            self.source_buffer.clear();
            self.stream = None;

            return ret_val;
        }
        else {
            None
        }
    }

    pub fn new() -> Listener {
        let mut listener = Listener {
            rtx: RTx::new(),
            socket: TcpListener::bind("0.0.0.0:42069").unwrap(),
            stream: None,
            source_buffer: String::new(),
            rx_buf: Buffer::new(),
            tx_buf: Buffer::new(),
        };

        listener.init();

        return listener;
    }
}
