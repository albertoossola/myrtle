use core::str::from_utf8;

use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::{path::Path, FsCommand}};
use alloc::string::{String, ToString};
use base64::Engine;

pub struct ReadCommandHandler;

const MAX_RW_LENGTH : usize = 64;

impl ShellCommandHandler for ReadCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.get(0)
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::CommandError("invalid path"))?;

        let from = args.get(1)
            .and_then(|arg| arg.parse().ok())
            .ok_or(ShellError::CommandError("invalid offset"))?;

        let length = args.get(2)
            .and_then(|arg| arg.parse().ok())
            .ok_or(ShellError::CommandError("invalid length"))?;

        if length > MAX_RW_LENGTH {
            return Err(ShellError::InvalidArgs);
        }

        let mut bytes_from_file = [0; MAX_RW_LENGTH];
        let mut bytes_length = 0;

        {
            let mut read_file_callback = |bytes: &[u8]| -> () { 
                bytes_from_file[..bytes.len()].copy_from_slice(bytes); 
                bytes_length = bytes.len();
            }; 
            let mut command = FsCommand::ReadFile(
                from, 
                length, 
                &mut read_file_callback
            );

            context.fs.run(&path, &mut command)
                .or(Err(ShellError::IOError))?;
        }

        //Send bytes as hex string
        for b in bytes_from_file[..bytes_length].iter() {
            let hex_alphabet = "0123456789ABCDEF".as_bytes();

            let hex_bytes = [
                hex_alphabet[(b >> 4 & 0x0F) as usize], 
                hex_alphabet[(b & 0x0F) as usize]
            ];

            let str = from_utf8(&hex_bytes).or(Err(ShellError::IOError))?;
            callback(&str);
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        String::from("read")
    }
}

impl ReadCommandHandler {
    pub fn new() -> Self {
        return ReadCommandHandler {}
    }
}