use crate::{Machine, MachineRunContext};

pub struct ProgramRunner {
    program: Option<Machine>,
}

impl ProgramRunner {
    pub fn new() -> ProgramRunner {
        ProgramRunner { program: None }
    }

    pub fn set_machine(&mut self, machine: Machine) {
        self.program = Some(machine);
    }

    pub fn stop(&mut self) {
        self.program = None;
    }

    pub fn run(&mut self, context: MachineRunContext) {
        match &mut self.program {
            Some(machine) => machine.run(context),
            None => {}
        }
    }
}
