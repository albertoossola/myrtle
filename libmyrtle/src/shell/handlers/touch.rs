use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::{FsCommand, path::Path}};
use alloc::string::String;

pub struct TouchCommandHandler;

impl ShellCommandHandler for TouchCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.get(0)
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::InvalidArgs)?;

        context.fs.run(&path, &mut FsCommand::MakeFile).or(Err(ShellError::InvalidArgs))
    }

    fn get_name(&self) -> String {
        String::from("touch")
    }
}

impl TouchCommandHandler {
    pub fn new() -> Self {
        Self {}
    }
}