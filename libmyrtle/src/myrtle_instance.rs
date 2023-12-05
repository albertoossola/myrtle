use crate::fs::ramfs::RamFs;
use crate::fs::rootfs::RootFs;
use crate::program_runner::ProgramRunner;
use crate::shell::Shell;
use crate::shell::command_handler::{EchoCommandHandler, ShellCommandHandler};
use crate::shell::context::ShellContext;
use crate::shell::handlers::ls::LsCommandHandler;
use crate::shell::handlers::mkdir::MkdirCommandHandler;
use crate::shell::handlers::read::ReadCommandHandler;
use crate::shell::handlers::rm::RmCommandHandler;
use crate::shell::handlers::run::RunCommandHandler;
use crate::shell::handlers::stop::StopCommandHandler;
use crate::shell::handlers::touch::TouchCommandHandler;
use crate::shell::handlers::write::WriteCommandHandler;
use crate::shell::shellio::ShellIO;
use crate::{HWAdapter, MachineRunContext, NodeData};
use alloc::boxed::Box;

pub struct MyrtleInstance {
    program_runner: ProgramRunner,
    hal: Box<dyn HWAdapter>,
    shell: Shell,
    io: Box<dyn ShellIO>,
    fs: RootFs
}

impl MyrtleInstance {
    fn sleep(&self, ms : u32) {
        let start_time = self.hal.get_ms_time();
        
        while self.hal.get_ms_time() - start_time < (ms as u64) { }
    }

    pub fn step(&mut self) {
        //TODO: This is a hack to get the shell to run for a bit
        for _ in 0..32 {
            let context = ShellContext { 
                hal: self.hal.as_mut(),
                fs: &mut self.fs,
                program_runner: &mut self.program_runner
            };        

            self.shell.update(self.io.as_mut(), context);
        }

        self.step_machine_if_present();
    }

    fn step_machine_if_present(&mut self) {
        let context = MachineRunContext {
            current_ticks: self.hal.get_ms_time(),
            current_ticks_us: self.hal.get_us_time(),
        };

        self.program_runner.run(context);
    }

    pub fn new(
        hw_adapter: Box<dyn HWAdapter>,
        shell_io: Box<dyn ShellIO>
    ) -> MyrtleInstance {
        let mut fs = RootFs::new();
        fs.mount("ramfs", Box::new(RamFs::new())).unwrap();

        let mut shell = Shell::new();
        shell
            .register_command_handler(Box::new(EchoCommandHandler))
            .register_command_handler(Box::new(MkdirCommandHandler))
            .register_command_handler(Box::new(LsCommandHandler))
            .register_command_handler(Box::new(RmCommandHandler))
            .register_command_handler(Box::new(WriteCommandHandler))
            .register_command_handler(Box::new(ReadCommandHandler))
            .register_command_handler(Box::new(TouchCommandHandler))
            .register_command_handler(Box::new(RunCommandHandler))
            .register_command_handler(Box::new(StopCommandHandler));

        return MyrtleInstance {
            hal: hw_adapter,
            program_runner: ProgramRunner::new(),
            io: shell_io,
            shell,
            fs
        };
    }
}
