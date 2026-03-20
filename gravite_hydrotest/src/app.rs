use crate::config::AppConfig;
use crate::gui::builder::GroupBuilder;
use crate::gui::pages::actuators::ActuatorsPage;
use crate::gui::pages::procedures::ProceduresPage;
use crate::gui::composite::{Component, ButtonState};
use crate::comm::{CommHandle, ToMcu, FromMcu};

#[derive(PartialEq)]
enum Page {
    Actuators,
    Procedures,
    Programming,
    SerialPortConnection,
}

pub struct App {
    current_page: Page,
    actuators: ActuatorsPage,
    procedures: ProceduresPage,
    comm: Option<CommHandle>
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { current_page: Page::Actuators, actuators: ActuatorsPage { 
            root: GroupBuilder::build_from_cfg(&config) }, 
            procedures: ProceduresPage::new(), 
            comm: None }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Some(comm) = &self.comm {
            while let Ok(msg) = comm.rx.try_recv() {
                match msg {
                    FromMcu::Echo(code, active) => {
                        let state = if active {
                            ButtonState::Active
                        } else {
                            ButtonState::Inactive
                        };
                        self.actuators.root.update_state(code, state);
                    }
                    _ => {}
                }
            }
        }

        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_page, Page::Actuators, "Aktuatory");
                ui.selectable_value(&mut self.current_page, Page::Procedures, "Procedury");
                ui.selectable_value(&mut self.current_page, Page::Programming, "Programowanie");
                ui.selectable_value(&mut self.current_page, Page::SerialPortConnection, "Port szeregowy");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Actuators => {
                    let clicked = self.actuators.show(ui);
                    if let Some(comm) = &self.comm {
                        for code in clicked {
                            let _ = comm.tx.send(ToMcu::TogglePeripheral(code));
                        }
                    }
                }
                Page::Procedures => { self.procedures.show(ui); }
                _ => {}
            }
        });
    }
}