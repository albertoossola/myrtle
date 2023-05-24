use std::net::{SocketAddr, UdpSocket};

use liblink::{Buffer, RTx, MAX_DATA};

pub struct Listener {
    pub rtx: RTx,
    tx_buf: liblink::Buffer,
    rx_buf: liblink::Buffer,

    socket: UdpSocket,
    sender: Option<SocketAddr>,
    source_buffer: String,
}

impl Listener {
    pub fn init(&mut self) {
        println!("Opening serial socket..");
        self.socket.set_nonblocking(true).unwrap();

        let mut sender: Option<SocketAddr> = None;

        println!("Listening for UDP packets");
    }

    pub fn update(&mut self) -> Option<String> {
        let mut received_frame: [u8; MAX_DATA] = [0 as u8; MAX_DATA];

        if self.rx_buf.can_write() {
            let mut buf = [0; 64];
            self.socket
                .recv_from(&mut buf)
                .and_then(|(l, addr)| match l {
                    0 => Ok(l),
                    _ => {
                        self.sender = Some(addr);

                        buf.iter().take(l).for_each(|b| self.rx_buf.write(*b));
                        Ok(l)
                    }
                })
                .unwrap_or(0);
        }

        self.tx_buf
            .read()
            .map(|txdata| self.sender.map(|addr| self.socket.send_to(&[txdata], addr)));

        let rec = self
            .rtx
            .update(&mut self.tx_buf, &mut self.rx_buf, &mut received_frame);

        return rec.and_then(|l| {
            self.rtx.poll_next();

            return core::str::from_utf8(&received_frame[..l])
                .map(|rec_str| match rec_str {
                    "@@clear" => {
                        println!("Receiving program...");
                        self.source_buffer.clear();
                        None
                    }
                    "@@run" => Some(self.source_buffer.clone()),
                    _ => {
                        self.source_buffer.push_str(rec_str);
                        None
                    }
                })
                .unwrap_or(None);
        });
    }

    pub fn new() -> Listener {
        let mut listener = Listener {
            rtx: RTx::new(),
            socket: UdpSocket::bind("0.0.0.0:42069").unwrap(),
            sender: None,
            source_buffer: String::new(),
            rx_buf: Buffer::new(),
            tx_buf: Buffer::new(),
        };

        listener.init();

        return listener;
    }
}
