use clap::Parser;

use liblink::{Buffer, RTx};

use std::{
    fs,
    io::{self, Read, Write},
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    thread::sleep,
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Myrtle file to upload
    #[arg(short, long)]
    file: String,

    /// UDP Address of the Myrtle instance
    #[arg(short, long)]
    address: Option<String>,
}

fn send_and_wait(
    rtx: &mut RTx,
    tx_buf: &mut Buffer,
    rx_buf: &mut Buffer,
    socket: &mut UdpSocket,
    to_send: &[u8],
) {
    rtx.send(to_send);

    print!(".");
    io::stdout().flush().ok();

    while !rtx.is_free() {
        rtx.update(tx_buf, rx_buf, &mut [0; 64]);

        tx_buf.read().map(|frame| socket.send(&[frame]));

        if rx_buf.can_write() {
            let mut recv_buf = [0];
            socket.recv(&mut recv_buf).map_or((), |len| match len {
                0 => {}
                _ => rx_buf.write(recv_buf[0]),
            });
        }

        sleep(Duration::from_micros(1000));
    }
}

fn send_file(file: &mut fs::File, addr: SocketAddr) -> io::Result<()> {
    let mut sock = std::net::UdpSocket::bind("0.0.0.0:0")?;
    sock.set_nonblocking(true).ok();

    println!("Socket bound to {}", sock.local_addr().unwrap());

    sock.connect(addr)?;

    println!("Socket connected to {}", sock.peer_addr().unwrap());

    let mut rx_buf = Buffer::new();
    let mut tx_buf = Buffer::new();

    let mut rtx = RTx::new();

    let mut send = |bytes: &[u8]| {
        send_and_wait(&mut rtx, &mut tx_buf, &mut rx_buf, &mut sock, bytes);
    };

    send("@@clear".as_bytes());

    loop {
        let mut buf = [0; 64];
        let read_result = file.read(&mut buf);

        match read_result {
            Ok(len) if len == 0 => break,
            Ok(len) => send(&buf[..len]),
            _ => break,
        }
    }

    send("@@run".as_bytes());

    Ok(())
}

fn main() {
    let args = Args::parse();

    let mut file = fs::File::open(args.file).expect(&"Could not open file");

    match args.address {
        Some(address) => {
            let sockaddr = SocketAddr::from_str(address.as_str()).expect("Invalid address");

            send_file(&mut file, sockaddr).unwrap();

            println!("Done!");
        }
        _ => {
            println!("No receiver specified");
        }
    }
}
