use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::{path::Path, FsCommand}};
use alloc::string::String;
use base64::Engine;

pub struct ReadCommandHandler;

impl ShellCommandHandler for ReadCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.get(0)
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::InvalidArgs)?;

        let from = args.get(1)
            .and_then(|arg| arg.parse().ok())
            .ok_or(ShellError::InvalidArgs)?;

        let length = args.get(2)
            .and_then(|arg| arg.parse().ok())
            .ok_or(ShellError::InvalidArgs)?;

        let mut send_as_base64 = |bytes: &[u8]| { 
            let engine = base64::engine::general_purpose::STANDARD_NO_PAD;
            let b64 = engine.encode(bytes);
            callback(b64.as_str()); 
        };

        let mut command = FsCommand::ReadFile(
            from, 
            length, 
            &mut send_as_base64
        );

        context.fs.run(&path, &mut command).or(Err(ShellError::InvalidCommand))
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