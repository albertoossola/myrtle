use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use crate::{HWAdapter, Machine, MachineRunContext, make_program, parse_program};
use crate::interface::channels::Channel;
use crate::interface::command_sources::CommandSource;
use crate::interface::Interface;

pub struct MyrtleInstance {
    machine: Option<Machine>,
    hal: Box<dyn HWAdapter>,
    interface: Interface,
}

impl MyrtleInstance {
    pub fn step(&mut self) {
        let new_source_or_none = self.poll_for_new_source();
        match new_source_or_none {
            Some(source) => self.parse_machine_and_run(&source),
            None => {}
        }

        self.step_machine_if_present();
    }

    fn poll_for_new_source(&mut self) -> Option<String> {
        return self.interface.poll();
    }

    fn parse_machine_and_run(&mut self, source: &str) {
        let ast_or_err = parse_program(source);

        match ast_or_err {
            Ok((_, mut ast)) => match make_program(self.hal.as_mut(), &mut ast) {
                Ok(machine) => self.machine = Some(machine),
                _ => {}
            },
            _ => {}
        }
    }

    fn step_machine_if_present(&mut self) {
        let context = MachineRunContext {
            current_ticks: 0,
            current_ticks_us: 0,
        };

        match self.machine.as_mut() {
            Some(machine) => machine.run(context),
            None => {}
        }
    }

    pub fn run_from_channel(&mut self, channel: u8) -> () {
        self.interface.set_channel(channel);
        let source_code_or_none = self.interface.get_channel_string_and_rewind();

        match source_code_or_none {
            Some(source_code) => self.parse_machine_and_run(&source_code),
            None => {}
        };
    }

    pub fn new(
        hw_adapter: Box<dyn HWAdapter>,
        command_source: Box<dyn CommandSource>,
        interface_channels: Vec<Box<dyn Channel>>
    ) -> MyrtleInstance {
        let mut interface = Interface::new(command_source, interface_channels);

        return MyrtleInstance {
            hal: hw_adapter,
            machine: None,
            interface,
        };
    }
}
