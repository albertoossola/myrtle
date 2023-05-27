use std::time::{SystemTime, UNIX_EPOCH};

use libmyrtle::{
    make_machine, make_program, parse_machine, parse_program, HWAdapter, Machine, MachineRunContext,
};

use crate::{
    hal,
    listener::{self, Listener},
    TestHal,
};

pub struct App {
    listener: Listener,
    machine: Option<Machine>,
    hal: TestHal,
}

impl App {
    fn update_listener(&mut self) -> std::result::Result<(), ()> {
        let listener_opt = self.listener.update();
        match listener_opt {
            Some(buf) => {
                //Set the machine to none to invoke the drop() methods;
                self.machine = None;

                let (_, mut ast) = parse_program(&buf).map_err(|_| ())?;

                self.machine = make_program(&mut self.hal, &mut ast).map_err(|_| ()).ok();
            }
            None => {}
        };

        Ok(())
    }

    fn update_stuff(&mut self) {
        for _ in 1..2 {
            self.update_listener().unwrap_or(());
        }

        let context = MachineRunContext {
            current_ticks: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            current_ticks_us: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        };

        match &mut self.machine {
            Some(machine) => machine.run(context),
            None => {}
        };
    }
}

impl App {
    pub fn update(&mut self) {
        self.update_stuff();
    }
}

impl Default for App {
    fn default() -> App {
        let mut hal_instance = hal::TestHal::new();
        hal_instance.init();

        App {
            listener: Listener::new(),
            machine: None,
            hal: hal_instance,
        }
    }
}
