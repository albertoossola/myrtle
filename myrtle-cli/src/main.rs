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

    _ = stream.write("@@clear".as_bytes());

    while !eof {
        match file.read(&mut buf) {
            Ok(0) => { eof = true; }
            Ok(len) => {
                _ = stream.write(&buf[..len]);
            },
            Err(_) => panic!("Errore while reading file")
        }
    }

    _ = stream.write("@@run".as_bytes());

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
