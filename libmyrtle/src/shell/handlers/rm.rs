use crate::{shell::{command_handler::ShellCommandHandler, ShellError}, fs::path::Path};
use alloc::string::String;


pub struct RmCommandHandler;

impl ShellCommandHandler for RmCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        _callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let path = args.first()
            .and_then(|arg| Path::new(arg).ok())
            .ok_or(ShellError::InvalidArgs)?;

        let command = &mut crate::fs::FsCommand::Delete;

        return context.fs.run(&path, command).or(Err(ShellError::InvalidCommand));
    }

    fn get_name(&self) -> String {
        return String::from("rm"); 
    }
}

impl RmCommandHandler {
    pub fn new() -> Self {
        return RmCommandHandler {}
    }
}