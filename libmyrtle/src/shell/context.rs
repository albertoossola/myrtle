use crate::{fs::FileSystem, program_runner::ProgramRunner, HWAdapter};

use super::command_handler::ShellCommandHandler;

pub struct ShellContext<'a> {
    pub fs: &'a mut dyn FileSystem,
    pub program_runner: &'a mut ProgramRunner,
    pub hal : &'a mut dyn HWAdapter
}
