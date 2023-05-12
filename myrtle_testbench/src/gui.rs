use std::time::{SystemTime, UNIX_EPOCH};

use eframe::*;
use libmyrtle::{
    make_machine, make_program, parse_machine, parse_program, HWAdapter, Machine, MachineRunContext,
};

use crate::{
    hal,
    listener::{self, Listener},
    TestHal,
};

pub struct GUI {
    listener: Listener,
    machine: Machine,
    hal: TestHal,
}

impl GUI {
    fn update_listener(&mut self) -> std::result::Result<(), ()> {
        let listener_opt = self.listener.update();
        match listener_opt {
            Some(buf) => {
                let (_, mut ast) = parse_program(&buf).map_err(|_| ())?;
                self.machine = make_program(&mut self.hal, &mut ast).map_err(|_| ())?;
            }
            None => {}
        };

        Ok(())
    }

    fn update_stuff(&mut self) {
        self.update_listener();

        let context = MachineRunContext {
            current_ticks: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        self.machine.run(context);
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for i in 0..10 {
            self.update_stuff();
        }

        ctx.set_pixels_per_point(2.0);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Myrtle Testbench");

            ui.horizontal(|ui| {
                for i in 0..10 {
                    let mut value = match self.hal.hw_state.borrow().pins.get(&i) {
                        Some(0) => false,
                        Some(1) => true,
                        _ => false,
                    };

                    ui.checkbox(&mut value, i.to_string());
                }
            });

            if ui.button("Quit").clicked() {
                frame.close()
            };
        });

        ctx.request_repaint();
    }
}

impl Default for GUI {
    fn default() -> GUI {
        let mut hal_instance = hal::TestHal::new();
        hal_instance.init();

        GUI {
            listener: Listener::new(),
            machine: Machine::make_blank(),
            hal: hal_instance,
        }
    }
}
