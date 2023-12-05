use crate::shell::{command_handler::ShellCommandHandler, context::ShellContext, ShellError};
use crate::fs::{path::Path, FsCommand};
use alloc::string::String;

pub struct LsCommandHandler;

impl ShellCommandHandler for LsCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        let arg1 = args.first().ok_or(ShellError::InvalidArgs)?;
        let path = Path::new(*arg1).map_err(|_| ShellError::InvalidArgs)?;

        _ = context
            .fs
            .run(&path, &mut FsCommand::GetDirs(&mut |dir| { callback(dir); callback("\r\n"); }))
            .or(Err(ShellError::InvalidArgs))?;

        _ = context
            .fs
            .run(&path, &mut FsCommand::GetFiles(&mut |file| { callback(file); callback("\r\n"); }))
            .or(Err(ShellError::InvalidArgs))?;

        Ok(())
    }

    fn get_name(&self) -> String {
        return String::from("ls");
    }
}