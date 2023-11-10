use crate::fs::{path::Path, FsCommand};

use super::{context::ShellContext, ShellError};
use alloc::string::String;

pub trait ShellCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), ShellError>;

    fn get_name(&self) -> String;
}

pub struct EchoCommandHandler;

impl ShellCommandHandler for EchoCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), ShellError> {
        let arg = args.first().ok_or(ShellError::InvalidArgs)?;
        callback(arg);

        Ok(())
    }

    fn get_name(&self) -> String {
        return String::from("echo");
    }
}

impl EchoCommandHandler {
    pub fn new() -> EchoCommandHandler {
        return EchoCommandHandler {};
    }
}
