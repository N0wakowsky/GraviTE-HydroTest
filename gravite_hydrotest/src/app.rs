use crate::{config::AppConfig, gui::{actuators::actuators_page::ActuatorsPage, procedures::{procedures_logic::ProcedureRunner, procedures_page::ProcedurePage}, serial_port::serial_port_logic::ConnectionState}};
use crate::gui::serial_port::serial_port_page::SerialPortPage;
use crate::gui::components::AppMessage;
use crate::config::ActuatorsRegister;
use crate::comm::states::FromMcu;

#[derive(PartialEq)]
enum Page {
    Actuators,
    Procedures,
    Programming,
    SerialPortPage,
}

pub struct App {
    current_page: Page,
    register: ActuatorsRegister,
    actuators: ActuatorsPage,
    serial_port: SerialPortPage,
    procedures: ProcedurePage,
    procedures_logic: ProcedureRunner,
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let register = ActuatorsRegister::from_config(&config);

        Self { 
            current_page: Page::Actuators, 
            actuators: ActuatorsPage::new(&config),
            serial_port: SerialPortPage::new(),
            procedures: ProcedurePage::new(&config),
            procedures_logic: ProcedureRunner::new(),
            register,
            config,
        }
    }
    
    pub fn handle_message(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::ConnectSerial { port, baud } => {
                let _ = self.serial_port.state.try_connect(&port, baud);
            }
            AppMessage::ToggleActuator(code) => {
                if let ConnectionState::Connected { handle } = &self.serial_port.state {
                    // Wysyłamy komendę do MCU
                    let _ = handle.send(crate::comm::states::ToMcu::TogglePeripheral(code));
                }
            }
            AppMessage::DisconnectSerial => {
                self.serial_port.state.try_disconnect();
                self.register.reset_all();
                self.actuators.root.reset_status();
            }
            AppMessage::StartProcedure(proc_name) => {
                // Znajdź konfigurację procedury i wystartuj silnik
                if let Some(proc_config) = self.config.procedures.iter().find(|p| p.name == proc_name) {
                    self.procedures_logic.start(proc_config.clone());
                }
            }
            AppMessage::AbortProcedure => {
                self.procedures_logic.stop();
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // 1. Odbieranie komunikatów z UART i synchronizacja ze "Źródłem Prawdy"
        let mut messages = Vec::new();
        if let ConnectionState::Connected { handle } = &self.serial_port.state {
            while let Some(msg) = handle.try_recv() {
                messages.push(msg);
            }
        }

        // 2. Teraz przetwórz wiadomości (handle już nie jest pożyczone)
        for msg in messages {
            match msg {
                FromMcu::Echo(code) => {
                    let new_state = self.register.toggle_state(code);
                    self.actuators.root.set_status_by_code(code, new_state);
                }
                FromMcu::Disconnected => {
                    self.handle_message(AppMessage::DisconnectSerial);
                }
            }
        }

        if self.procedures_logic.is_running() {
            let toggle_commands = self.procedures_logic.tick(&self.register);
            
            if let ConnectionState::Connected { handle } = &self.serial_port.state {
                for code in toggle_commands {
                    let _ = handle.send(crate::comm::states::ToMcu::TogglePeripheral(code));
                }
            }
            ctx.request_repaint(); 
        }

        // Toolbar
        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_page, Page::Actuators, "Actuators");
                ui.selectable_value(&mut self.current_page, Page::SerialPortPage, "Serial Port");
                ui.selectable_value(&mut self.current_page, Page::Procedures, "Procedures");
            });
        });

        //Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            let maybe_msg = match self.current_page {
                Page::Actuators => self.actuators.show(ui),
                Page::SerialPortPage => SerialPortPage::show(ui, &mut self.serial_port.state),
                Page::Procedures => self.procedures.show(ui),
                _ => None,
            };

            if let Some(msg) = maybe_msg {
                self.handle_message(msg);
            }
        });
    }
}