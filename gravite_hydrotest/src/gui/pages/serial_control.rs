use std::{ thread, time::Duration};

use egui::Color32;
use serialport::SerialPort;

use crate::{config::ActuatorsRegister, gui::components::PageTrait};
use crate::gui::components::PageContext;
use crate::gui::components::AppState;

use std::sync::mpsc;

pub enum SerialCommand {
    RefreshPorts,
    Connect {port: String, baudrate: u32},
    Disconnect,
    TogglePeripheral(u8)
}

pub enum SerialStatus {
    Disconnected,
    Connected(String),
    Error(String),
    PortList(Vec<String>)
}

fn get_avaiable_ports() -> Vec<String> {
    match serialport::available_ports() {
        Ok(ports) => ports.into_iter().map(|p| p.port_name).collect(),
        Err(_) => vec![],
    }
}

pub fn spawn_serial_thread(
    rx_command: mpsc::Receiver<SerialCommand>, 
    tx_status: mpsc::Sender<SerialStatus>, 
    act_register: ActuatorsRegister,
    ctx: egui::Context
) {
    thread::spawn(move || {
        let mut serial: Option<Box<dyn SerialPort>> = None;
        let _ = tx_status.send(SerialStatus::PortList(get_avaiable_ports()));

        loop {
            if let Ok(cmd) = rx_command.try_recv() {
                match cmd {
                    SerialCommand::Connect { port, baudrate } => {
                        match serialport::new(&port, baudrate).timeout(Duration::from_millis(50)).open() {
                            Ok(p) => {
                                serial = Some(p);
                                let _ = tx_status.send(SerialStatus::Connected(port));
                            }
                            Err(e) => { let _ = tx_status.send(SerialStatus::Error(e.to_string())); }
                        }
                    }
                    SerialCommand::TogglePeripheral(code) => {
                        if let Some(port) = &mut serial {
                            if port.write(&[code]).is_ok() {
                                let mut buf = [0u8; 1];
                                if port.read(&mut buf).is_ok() && buf[0] == code {
                                    act_register.toggle_state(code);
                                }
                            }
                        }
                    }
                    SerialCommand::Disconnect => { serial = None; let _ = tx_status.send(SerialStatus::Disconnected); }
                    SerialCommand::RefreshPorts => { let _ = tx_status.send(SerialStatus::PortList(get_avaiable_ports())); }
                }
                ctx.request_repaint();
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
}



// Page implementation

pub struct SerialPage {
    tx_command: mpsc::Sender<SerialCommand>,
    selected_port: String,
    baud_rate: u32,
    baud_rates: Vec<u32>
}

impl SerialPage {
    pub fn new(ctx: &PageContext) -> Self {
        Self {
            tx_command: ctx.tx_serial.clone(),
            selected_port: String::new(),
            baud_rate: 115200,
            baud_rates: vec![9600, 19200, 38400, 57600, 115200]
        }
    }
}

impl PageTrait for SerialPage {
    fn update(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &AppState) {
        ui.heading("UART Configurator");
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Status: ");
            match &state.serial_status {
                SerialStatus::Disconnected => ui.colored_label(egui::Color32::GRAY, "Disconnected"),
                SerialStatus::Connected(port) => ui.colored_label(egui::Color32::GREEN, format!("Connected to {}", port)),
                SerialStatus::Error(e) => ui.colored_label(Color32::RED, format!("Error: {}", e)),
                _ => ui.label(""),
            }
        });

        ui.add_space(20.0);

        let is_connected = matches!(state.serial_status, SerialStatus::Connected(_));

        ui.add_enabled_ui(!is_connected, |ui|{
            ui.horizontal(|ui|{
                ui.label("Port:");
                egui::ComboBox::from_id_salt("port_combo")
                    .selected_text(&self.selected_port)
                    .show_ui(ui, |ui|{
                        for port in &state.available_ports {
                            ui.selectable_value(&mut self.selected_port, port.clone(), port);
                        }

                });

                if ui.button("Refresh").clicked() {
                    let _ = self.tx_command.send(SerialCommand::RefreshPorts);
                }

            });

            ui.horizontal(|ui|{
                ui.label("Baudrate:");
                egui::ComboBox::from_id_salt("baud_combo")
                    .selected_text(&self.baud_rate.to_string())
                    .show_ui(ui, |ui|{
                        for &baud in &self.baud_rates {
                            ui.selectable_value(&mut self.baud_rate, baud, baud.to_string());
                        }

                });
            });
        });

        ui.horizontal(|ui| {
            if !is_connected {
                if ui.button("Connect").clicked() && !self.selected_port.is_empty() {
                    let _ = self.tx_command.send(SerialCommand::Connect { 
                        port: self.selected_port.clone(), 
                        baudrate: self.baud_rate
                    });
                }
            } else {
                if ui.button("Disconnect").clicked() {
                    let _ = self.tx_command.send(SerialCommand::Disconnect);
                }
            }
        });
    }
}