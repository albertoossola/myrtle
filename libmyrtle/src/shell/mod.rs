pub mod command_handler;
mod prompt;

pub mod context;
pub mod handlers;
pub mod shellio;
use context::ShellContext;

use alloc::{boxed::Box, vec::Vec};
use alloc::string::String;
use nom::AsChar;

use self::{prompt::ShellPrompt, command_handler::ShellCommandHandler, shellio::ShellIO};

#[derive(Debug, PartialEq)]
pub enum ShellError {
    InvalidCommand,
    InvalidArgs,
    IOError
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
                self.run_command(
                    context, 
                    &mut |out : &str| {
                        for char in out.as_bytes() {
                            while io.write(*char).is_err() {}
                        }
                    }
                ).ok();

                self.buffer.clear();
            },
            Some(0x08) => { self.buffer.pop(); },
            Some(c) if self.buffer.len() < 64 => self.buffer.push(c.as_char()),
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

#[cfg(test)]
mod test {
    use crate::shell::context::ShellContext;
    use crate::{fs::ramfs::RamFs, program_runner::ProgramRunner};

    use super::{command_handler::EchoCommandHandler, Shell, ShellError};

    fn run_command_with_context(
        shell: &mut Shell,
        prompt_text: &str,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), ShellError> {
        let context = ShellContext {
            fs: &mut RamFs::new(),
            program_runner: &mut ProgramRunner::new()
        };

        shell.buffer = String::from(prompt_text);
        return shell.run_command(context, callback);
    }

    #[test]
    pub fn test_empty_command() {
        let mut shell = Shell::new();

        assert!(run_command_with_context(&mut shell, "", &mut |_| {}).is_err());
    }

    #[test]
    pub fn test_non_existent_command() {
        let mut shell = Shell::new();

        assert!(run_command_with_context(&mut shell, "", &mut |_| {}).is_err());
    }

    #[test]
    pub fn text_invalid_args() {
        let mut shell = Shell::new();
        shell.register_command_handler(Box::new(EchoCommandHandler::new()));

        assert_eq!(
            Err(ShellError::InvalidArgs),
            run_command_with_context(&mut shell, "echo", &mut |_| {})
        );
    }

    #[test]
    pub fn text_valid_command() {
        let mut shell = Shell::new();
        shell.register_command_handler(Box::new(EchoCommandHandler::new()));

        let mut output: String = String::new();

        run_command_with_context(&mut shell, "echo foo", &mut |s| output = String::from(s))
            .unwrap();

        assert_eq!("foo", output);
    }
}
