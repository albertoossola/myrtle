use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::interface::command_sources::{Command, CommandSource};

pub struct TcpIpCommandSource {
    tcp_listener: TcpListener,
    tcp_stream: Option<TcpStream>,
}

impl CommandSource for TcpIpCommandSource {
    fn poll(&mut self) -> Option<Command> {
        let command_or_none = self.poll_socket_for_command();

        return match command_or_none {
            Some(command) => Some(command),
            None => None,
        };
    }
}

impl TcpIpCommandSource {
    fn poll_socket_for_command(&mut self) -> Option<Command> {
        match self.tcp_stream {
            Some(_) => self.poll_byte_from_stream(),
            None => {
                self.accept_tcp_connection();
                return None;
            }
        }
    }

    fn accept_tcp_connection(&mut self) {

        match self.tcp_listener.accept() {
            Ok((stream, _)) => {
                self.tcp_stream = Some(stream)
            },
            _ => {}
        }
    }

    fn poll_byte_from_stream(&mut self) -> Option<Command> {
        let mut receive_buffer : [u8; 1] = [0; 1];

        return self
            .tcp_stream
            .as_mut()
            .and_then(|stream| stream.read_exact(&mut receive_buffer).ok())
            .and_then(|_| self.interpret_byte(receive_buffer[0]));
    }

    fn interpret_byte(&mut self, byte: u8) -> Option<Command> {
        match byte {
            0x08 => {
                self.tcp_stream = None;
                Some(Command::Run)
            },
            0x10 => Some(Command::SetChannel(0)),
            _ => Some(Command::Write(byte)),
        }
    }

    pub fn new(address : &str) -> Result<TcpIpCommandSource, ()> {
        let mut listener = TcpListener::bind(address).map_err(|_| ())?;
        listener.set_nonblocking(true).ok();

        let command_source = TcpIpCommandSource {
            tcp_listener: listener,
            tcp_stream: None,
        };

        return Ok(command_source);
    }
}
