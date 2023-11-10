use crate::shell::{command_handler::ShellCommandHandler, context::ShellContext, ShellError};
use crate::fs::{path::Path, FsCommand};
use alloc::string::String;


pub struct MkdirCommandHandler;

impl ShellCommandHandler for MkdirCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: ShellContext,
        _callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), ShellError> {
        let arg1 = args.first().ok_or(ShellError::InvalidArgs)?;
        let path = Path::new(*arg1).map_err(|_| ShellError::InvalidArgs)?;

        return context
            .fs
            .run(&path, &mut FsCommand::MakeDir)
            .map_err(|_| ShellError::InvalidArgs);
    }

    fn get_name(&self) -> String {
        return String::from("mkdir");
    }
}