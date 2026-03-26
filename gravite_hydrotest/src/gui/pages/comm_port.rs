use eframe::egui;
use crate::comm::serial::{Connected, Disconnected, SerialHandle};


pub enum SerialAction {
    Connect { port: String, baud_rate: u32},
    Disconnect,
    None,
}

pub enum ConnectionState {
    Disconnected {
        ports: Vec<String>,
        selected: usize,
        baud_rate: u32,
        last_error: Option<String>,
    },
    Connected {
        handle: SerialHandle<Connected>,
    },
}

fn list_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.port_name)
        .collect()
}

impl ConnectionState {
    pub fn new() -> Self {
        ConnectionState::Disconnected { ports: list_ports(), selected: 0, baud_rate: 115200, last_error: None }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> SerialAction {
        match self {
            ConnectionState::Disconnected { ports, selected, baud_rate, last_error } => {
                ui.heading("Serial Port");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Port:");
                    egui::ComboBox::from_id_salt("port select")
                        .selected_text(
                            ports.get(*selected).map(|s| s.as_str()).unwrap_or("No ports deteced")
                        )
                        .show_ui(ui, |ui| {
                            for (i, port) in ports.iter().enumerate() {
                                ui.selectable_value(selected, i, port);
                            }
                        });

                        if ui.button("Refresh").clicked() {
                            *ports = list_ports();
                            *selected = 0;
                        }
                });

                ui.horizontal(|ui| {
                    ui.label("Baudrate");
                    egui::ComboBox::from_id_salt("baud select")
                        .selected_text(baud_rate.to_string())
                        .show_ui(ui, |ui| {
                            for &rate in &[9600u32, 19200, 38400, 57600, 115200, 230400] {
                                ui.selectable_value(baud_rate, rate, rate.to_string());
                            }
                        });
                });

                if let Some(err) = last_error {
                    ui.colored_label(egui::Color32::RED, err.as_str());
                }

                if ui.button("Connect").clicked() {
                    let port = ports.get(*selected).cloned().unwrap_or_default();
                    return SerialAction::Connect { port: port.clone(), baud_rate: *baud_rate };
                }

                SerialAction::None
            }

            ConnectionState::Connected { handle } => {
                ui.heading("Serial Port");
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(50, 200, 50),
                format!("Connected: {}", handle.port_name));

                if ui.button("Disconnect").clicked() {
                    return SerialAction::Disconnect;
                }

                SerialAction::None
            }
        }
    }
}   
