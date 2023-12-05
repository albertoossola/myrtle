use clap::{command, Parser};

use std::{
    fs,
    io::{self, Read, Write},
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    thread::sleep,
    time::Duration,
};

use serialport::{self, SerialPort};
use std::net::TcpStream;

const BACKSPACE_BYTE : u8 = 0x08;
const LINE_FEED_BYTE : u8 = 0x10;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Myrtle file to upload
    #[arg(short, long)]
    file: String,

    /// Serial port path of the receiver
    #[arg(short, long)]
    port: Option<String>,
}

fn wait_for_ack(serial : &mut dyn SerialPort) -> io::Result<()> {
    let mut buf = [0; 1];

    while serial.read(&mut buf)? == 0 {
        sleep(Duration::from_millis(1));
    }

    while buf[0] != '>' as u8 {
        serial.read(&mut buf)?;
    }

    serial.clear(serialport::ClearBuffer::Input)?;

    Ok(())
}

fn send_file(file: &mut fs::File, serial_path: &str) -> io::Result<()> {
    /*Open a connection to the serial port */
    let mut port = serialport::new(serial_path, 115200)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(1000))
        .open()?;

    port.clear(serialport::ClearBuffer::All)?;
    //Send backspaces to clear the buffer

    port.flush()?;

    for _ in 0..64 {
        _ = port.write(&[BACKSPACE_BYTE]);
    }

    port.flush()?;

    port.write("\r".as_bytes())?;
    port.flush()?;

    wait_for_ack(port.as_mut())?;

    port.write("stop\r".as_bytes())?;
    port.flush()?;
    wait_for_ack(port.as_mut())?;

    port.write("echo start\r".as_bytes())?;
    port.flush()?;
    wait_for_ack(port.as_mut())?;

    port.write("rm ramfs/boot\r".as_bytes())?;
    port.flush()?;
    wait_for_ack(port.as_mut())?;

    port.write("touch ramfs/boot\r".as_bytes())?;
    port.flush()?;
    wait_for_ack(port.as_mut())?;

    let mut file_content = vec![]; 
    file.read_to_end(&mut file_content)?;

    send_bytes_through_port(port.as_mut(), &file_content);

    port.write("run ramfs/boot\r".as_bytes())?;
    port.flush()?;
    wait_for_ack(port.as_mut())?;

    Ok(())
}

fn send_bytes_through_port(port : &mut dyn SerialPort, bytes : &[u8]) {
    println!("Sending {} bytes", bytes.len());
    let mut sent = 0;

    for byte in bytes.iter() {
        if *byte == BACKSPACE_BYTE || *byte == LINE_FEED_BYTE || *byte == '\r' as u8 {
            continue;
        }

        let command = format!("write ramfs/boot {}\r", byte);
        _ = port.write(command.as_bytes());
        port.flush().ok();
        wait_for_ack(port).ok();

        print!("\r{:.0}%", (sent as f32 / bytes.len() as f32) * 100.0);
        sent += 1;
        io::stdout().flush().ok();
    }
}

fn main() {
    let args = Args::parse();

    let mut file = fs::File::open(args.file).expect(&"Could not open file");

    match args.port {
        Some(port) => {
            send_file(&mut file, &port).unwrap();

            println!("Done!");
        }
        _ => {
            println!("No receiver specified");
        }
    }
}
