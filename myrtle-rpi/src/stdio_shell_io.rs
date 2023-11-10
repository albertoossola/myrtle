use std::{net::{TcpStream, TcpListener}, io::{Read, Write}};

use libmyrtle::shell::{shellio::ShellIO, ShellError};

pub struct StdioShellIO {
    listener : TcpListener,
    socket_or_none : Option<TcpStream>
}

impl ShellIO for StdioShellIO {
    fn write(&mut self, c : u8) -> Result<(), libmyrtle::shell::ShellError> {
        if let Some(socket) = self.get_socket() {
            return socket.write(&[c])
                .map(|_| ())
                .or(Err(ShellError::IOError));
        }
        else {
            Err(ShellError::IOError)
        }
    }

    fn read(&mut self) -> Option<u8> {
        if let Some(socket) = self.get_socket() {
            let mut buffer = [ 0 ];
            
            return match socket.read(&mut buffer) {
                Ok(size) if size == 1 => {
                    return Some(buffer[0])
                },
                _ => {
                    None
                }
            };
        }

        return None;

    }
}

impl StdioShellIO {
    fn get_socket(&mut self) -> Option<&mut TcpStream> {
        match self.socket_or_none.is_some() {
            true => self.socket_or_none.as_mut(),
            false => {
                self.socket_or_none = self.accept_connection();
                return self.socket_or_none.as_mut();
            }
        }   
    }

    fn accept_connection(&mut self) -> Option<TcpStream> {
        self.listener
            .accept()
            .map(|(stream, _)| stream)
            .ok()
    }
}

impl StdioShellIO {
    pub fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:42069").unwrap();
        listener.set_nonblocking(true);

        StdioShellIO {  
            listener,
            socket_or_none: None
        }
    }
}