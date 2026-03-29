use eframe::egui;
use crate::gui::components::AppMessage;
use super::serial_port_logic::ConnectionState;

pub struct SerialPortPage {
    pub state: ConnectionState,
}

impl SerialPortPage {
    pub fn new() -> Self {
        Self { state: ConnectionState::new() }
    }

    pub fn show(ui: &mut egui::Ui, state: &mut ConnectionState) -> Option<AppMessage> {
        let mut message = None;

        ui.heading("Serial Port Settings");
        ui.separator();

        match state {
            ConnectionState::Disconnected { ports, selected, baud_rate, last_error } => {
                ui.horizontal(|ui| {
                    ui.label("Port:");
                    egui::ComboBox::from_id_salt("port_select")
                        .selected_text(ports.get(*selected).unwrap_or(&"None".to_string()))
                        .show_ui(ui, |ui| {
                            for (i, port) in ports.iter().enumerate() {
                                ui.selectable_value(selected, i, port);
                            }
                        });

                    if ui.button("Refresh").clicked() {
                        *ports = ConnectionState::list_ports();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Baudrate:");
                    egui::ComboBox::from_id_salt("baud_select")
                        .selected_text(baud_rate.to_string())
                        .show_ui(ui, |ui| {
                            for &rate in &[9600, 115200, 230400] {
                                ui.selectable_value(baud_rate, rate, rate.to_string());
                            }
                        });
                });

                if let Some(err) = last_error {
                    ui.colored_label(egui::Color32::RED, err);
                }

                if ui.button("Connect").clicked() {
                    if let Some(port) = ports.get(*selected) {
                        message = Some(AppMessage::ConnectSerial { 
                            port: port.clone(), 
                            baud: *baud_rate 
                        });
                    }
                }
            }
            ConnectionState::Connected { handle } => {
                ui.colored_label(egui::Color32::GREEN, format!("Connected to {}", handle.port_name));
                if ui.button("Disconnect").clicked() {
                    message = Some(AppMessage::DisconnectSerial);
                }
            }
        }

        message
    }
}