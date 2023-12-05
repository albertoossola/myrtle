use core::mem::size_of;

use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::{path::Path, FsCommand, FileSystem}, parse_program, converter};
use alloc::string::String;
use base64::Engine;


pub struct RunCommandHandler;

impl ShellCommandHandler for RunCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.get(0)
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::InvalidArgs)?;

        callback("loading..");

        let source = Self::read_whole_file(context.fs, &path)
            .or(Err(ShellError::CommandError("can't open file")))?;

        callback(&source);
        callback("parsing..");

        let (_, mut program_ast) = parse_program(&source)
            .or(Err(ShellError::CommandError("parse error")))?;

        let machine = converter::make_program(context.hal, &mut program_ast).or(Err(ShellError::InvalidArgs))?;

        context.program_runner.set_machine(machine);

        callback("ok");

        Ok(())
    }

    fn get_name(&self) -> String {
        String::from("run")
    }
}

impl RunCommandHandler {
    pub fn new() -> Self {
        Self { }    
    }

    fn read_whole_file(fs : &mut dyn FileSystem, path : &Path) -> Result<String, ShellError> {
        let mut buffer = String::new();
        let mut cursor = 0;

        loop {
            let bytes_read = Self::read_chunk_into_buffer(fs, path, &mut buffer, cursor, 1);

            cursor += bytes_read;
            
            if bytes_read == 0 {
                break;
            }
        }

        return Ok(buffer);
    }

    fn read_chunk_into_buffer(fs : &mut dyn FileSystem, path : &Path, buffer : &mut String, cursor : usize, length : usize) -> usize { 
        let mut bytes_read = 0;
        let mut callback = |bytes : &[u8]| {
            core::str::from_utf8(bytes)
                .and_then(|slice| {
                    buffer.push_str(slice);
                    bytes_read = slice.len() as usize;

                    Ok(())
                }
            ).ok();
        };

        let mut command = FsCommand::ReadFile(cursor, length, &mut callback);

        fs.run(path, &mut command).ok();

        bytes_read
    }
}