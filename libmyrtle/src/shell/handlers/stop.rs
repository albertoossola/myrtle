use crate::shell::command_handler::ShellCommandHandler;
use alloc::string::String;

pub struct StopCommandHandler;

impl ShellCommandHandler for StopCommandHandler {
    fn run(
        &mut self,
        args: &[&str],
        context: crate::shell::context::ShellContext,
        callback: &mut dyn FnMut(&str) -> (),
    ) -> Result<(), crate::shell::ShellError> {
        context.program_runner.stop();
        callback("stopped");

        Ok(())
    }

    fn get_name(&self) -> String {
        String::from("stop")
    }
}

impl StopCommandHandler {
    pub fn new() -> Self {
        Self { }    
    }
}