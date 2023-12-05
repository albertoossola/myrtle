use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::path::Path};
use alloc::string::String;
use base64::Engine;

pub struct WriteCommandHandler;

impl ShellCommandHandler for WriteCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.get(0)
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::CommandError("path required"))?;

        let data = args.get(1)
            .and_then(|arg| arg.parse().ok())
            .ok_or(ShellError::CommandError("data required"))?;

        let buffer = [data];
        let mut command = crate::fs::FsCommand::AppendToFile(&buffer);
        context.fs.run(&path, &mut command).or(Err(ShellError::IOError))
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