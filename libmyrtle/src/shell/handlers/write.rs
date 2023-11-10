use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::path::Path};
use alloc::string::String;
use base64::Engine;

pub struct WriteCommandHandler;

impl ShellCommandHandler for WriteCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        _callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.get(0)
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::InvalidArgs)?;

        let data = args.get(1)
            .ok_or(ShellError::InvalidArgs)?;

        let b64_engine = base64::engine::general_purpose::STANDARD_NO_PAD;
        let mut bytes = [0; 64];

        let bytes_written = b64_engine.decode_slice(data, &mut bytes).or(Err(ShellError::InvalidArgs))?;
        
        let command = &mut crate::fs::FsCommand::AppendToFile(&bytes[..bytes_written]);
        context.fs.run(&path, command).or(Err(ShellError::IOError))
    }

    fn get_name(&self) -> String {
        String::from("write")
    }
}

impl WriteCommandHandler {
    pub fn new() -> Self {
        Self {}
    }
}