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
use std::net::TcpStream;

const BACKSPACE_BYTE : u8 = 0x08;
const LINE_FEED_BYTE : u8 = 0x10;

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

fn send_file(file: &mut fs::File, addr: SocketAddr) -> io::Result<()> {
    let mut stream = TcpStream::connect(addr).expect("Could not connect to the client");

    let mut buf = [0; 64];
    let mut eof = false;

    //Send backspace to clear the buffer
    _ = stream.write(&[BACKSPACE_BYTE]);

    while !eof {
        match file.read(&mut buf) {
            Ok(0) => { eof = true; }
            Ok(len) => send_bytes_through_stream(&mut stream, &buf[..len]),
            Err(_) => panic!("Error while reading file")
        }
    }

    //Send line feed to commit changes
    _ = stream.write(&[LINE_FEED_BYTE]);

    Ok(())
}

fn send_bytes_through_stream(stream : &mut TcpStream, bytes : &[u8]) {
    for byte in bytes {
        if *byte == BACKSPACE_BYTE || *byte == LINE_FEED_BYTE {
            continue;
        }

        _ = stream.write(&[*byte]);
    }
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
