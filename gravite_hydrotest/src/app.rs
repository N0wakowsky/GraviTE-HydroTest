use std::clone;

use crate::comm::serial::SerialHandle;
use crate::config::AppConfig;
use crate::gui::builder::GroupBuilder;
use crate::gui::pages::actuators::{self, ActuatorsPage};
use crate::gui::pages::comm_port::{ConnectionState, SerialAction};
use crate::gui::pages::procedures::ProceduresPage;
use crate::gui::composite::{Component, ButtonState};
use crate::comm::states::{ToMcu, FromMcu};

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
    comm: ConnectionState,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { current_page: Page::Actuators, actuators: ActuatorsPage { 
            root: GroupBuilder::build_from_cfg(&config) }, 
            procedures: ProceduresPage::new(), 
            comm: ConnectionState::new(), }
    }

    fn try_connect(&mut self, port: &str, baud: u32) {
        let handle = SerialHandle::new(port, baud);
        match handle.connect() {
            Ok(connected) => {
                self.comm = ConnectionState::Connected { handle: connected };
            }
            Err(e) => {
                if let ConnectionState::Disconnected { last_error, .. } = &mut self.comm {
                    *last_error = Some(e.to_string());
                }
            }
        }
    }

    fn try_disconnect(&mut self) {
        let old_state = std::mem::replace(&mut self.comm, ConnectionState::new());

        if let ConnectionState::Connected { handle } = old_state {
            let disconnected_port_name = handle.port_name.clone();

            self.actuators.root.reset_status();
            
            let _ = handle.disconnect();

            if let ConnectionState::Disconnected { ports, selected, .. } = &mut self.comm {
                if let Some(pos) = ports.iter().position(|p| p == &disconnected_port_name) {
                    *selected = pos;
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let messages: Vec<FromMcu> = if let ConnectionState::Connected { handle } = &self.comm {
            std::iter::from_fn(|| handle.try_recv()).collect()
        } else {
            vec![]
        };

        let mut should_disconnect = false;

        for msg in messages {
            match msg {
                FromMcu::Echo(code) => {
                    self.actuators.root.update_state(code);
                }
                FromMcu::Disconnected => {
                    should_disconnect = true;
                }
                _ => {}
            }
        }

        if should_disconnect {
            self.try_disconnect();
        }

        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_page, Page::Actuators, "Actuators");
                ui.selectable_value(&mut self.current_page, Page::Procedures, "Procedures");
                ui.selectable_value(&mut self.current_page, Page::Programming, "Flash memory");
                ui.selectable_value(&mut self.current_page, Page::SerialPortConnection, "Serial port");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Actuators => {
                    let clicked = self.actuators.show(ui);
                    if let ConnectionState::Connected { handle } = &self.comm {
                        for code in clicked {
                            let _ = handle.send(ToMcu::TogglePeripheral(code));
                        }
                    }
                }
                Page::Procedures => { self.procedures.show(ui); }
                Page::SerialPortConnection => {
                    match self.comm.show(ui) {
                        SerialAction::Connect { port, baud_rate } => {
                            self.try_connect(&port, baud_rate);
                        }
                        SerialAction::Disconnect => {
                            self.try_disconnect();
                        }
                        SerialAction::None => {}
                    }
                }
                _ => {}
            }
        });
    }
}