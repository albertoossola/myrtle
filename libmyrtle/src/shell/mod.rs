pub mod command_handler;
mod prompt;

pub mod context;
pub mod handlers;
pub mod shellio;
pub mod datasource_shellio;
use context::ShellContext;

use alloc::{boxed::Box, vec::Vec};
use alloc::string::String;
use nom::AsChar;

use self::{prompt::ShellPrompt, command_handler::ShellCommandHandler, shellio::ShellIO};

#[derive(Debug, PartialEq)]
pub enum ShellError {
    InvalidCommand,
    InvalidArgs,
    IOError,
    CommandError(&'static str)
}

pub struct Shell {
    command_handlers: Vec<Box<dyn ShellCommandHandler>>,
    buffer : String
}

impl Shell {
    fn run_command(
        &mut self,
        context: ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), ShellError> {
        let prompt = ShellPrompt::parse(&self.buffer)?;

        let command_name = prompt.get_command();
        let args = prompt.get_args();

        let command_handler = self
            .command_handlers
            .iter_mut()
            .find(|h| h.get_name() == command_name)
            .ok_or(ShellError::InvalidCommand)?;

        return command_handler.run(args, context, callback);
    }

    pub fn update(&mut self, io : &mut dyn ShellIO, context: ShellContext) {
        match io.read() {
            Some(0x0D) => {
                io.write('\r' as u8).ok();
                io.write('\n' as u8).ok();

                self.run_command(
                    context, 
                    &mut |out : &str| {
                        for char in out.as_bytes() {
                            while io.write(*char).is_err() {}
                        }
                    }
                ).or_else(|err| match err {
                    ShellError::CommandError(err) => {
                        for char in err.as_bytes() {
                            while io.write(*char).is_err() {}
                        }

                        Ok(())
                    },
                    ShellError::InvalidCommand => {
                        for char in "Invalid command".as_bytes() {
                            while io.write(*char).is_err() {}
                        }

                        Ok(())
                    },
                    ShellError::InvalidArgs => {
                        for char in "Invalid arguments".as_bytes() {
                            while io.write(*char).is_err() {}
                        }

                        Ok(())
                    },
                    _ => io.write('!' as u8)
                })
                .ok();

                io.write('\r' as u8).ok();
                io.write('\n' as u8).ok();
                io.write('>' as u8).ok();

                self.buffer.clear();
            },
            Some(0x08) => { 
                self.buffer.pop(); 
                io.write(0x08 as u8).ok();
            },
            Some(c) if self.buffer.len() < 64 => {
                self.buffer.push(c.as_char());
                io.write(c as u8).ok();
            }
            _ => {}
        }


    }

    pub fn register_command_handler(&mut self, command_handler: Box<dyn ShellCommandHandler>) -> &mut Self {
        self.command_handlers.push(command_handler);
        self
    }

    pub fn new() -> Shell {
        let mut shell = Shell {
            command_handlers: Vec::new(),
            buffer: String::new()
        };

        shell.buffer.reserve_exact(64);

        return shell;
    }
}