use std::sync::mpsc;

use egui::{Color32, Vec2};

use crate::config::ActuatorsRegister;
use crate::gui::pages::serial_control::SerialCommand;
use crate::gui::components::ButtonTrait;
use crate::gui::components::PageTrait;

use crate::gui::components::PageContext;
use crate::gui::components::AppState;

use crate::config::Actuator;


// button implementation
struct ActButton {
    code: u8,
    label: String,
    act_register: ActuatorsRegister,
    tx_serial: mpsc::Sender<SerialCommand>,
}

impl ActButton {
    fn new(actuator: Actuator, act_register: ActuatorsRegister, tx_serial: mpsc::Sender<SerialCommand>) -> Self {
        let label = actuator.name;

        Self { 
            code: actuator.code, 
            label,
            act_register,
            tx_serial
        }
    }
}


impl ButtonTrait for ActButton {
    fn label(&self) -> &str { &self.label }
    fn size(&self) -> Vec2 { Vec2::new(120.0, 50.0) }
    fn color(&self, ui: &egui::Ui) -> Color32 {
        let items = self.act_register.items.lock();
        if items.iter().find(|a| a.code == self.code).map_or(false, |a| a.is_active) {
            Color32::from_rgb(0, 150, 0)
        } else {
            ui.visuals().widgets.noninteractive.bg_fill
        }
    }
    fn on_click(&mut self) {
        let _ = self.tx_serial.send(SerialCommand::TogglePeripheral(self.code));
    }
}

struct ActModule {
    name: String,
    buttons: Vec<Box<ActButton>>
}

impl ActModule {
    pub fn new(name: String) -> Self {
        Self { name, buttons: Vec::new() }
    }

    fn add_button(&mut self, button: Box<ActButton>) {
        self.buttons.push(button);
    }
}


// page implementation
pub struct ActPage {
    modules: Vec<ActModule>,
}

impl ActPage {
    pub fn new(ctx: &PageContext) -> Self {

        let mut modules = Vec::new();     
        for act_config in &ctx.config.actuators {
            let mut module = ActModule::new(act_config.module.clone());

            for actuator in &act_config.buttons {
                let button = ActButton::new(actuator.clone(), ctx.act_register.clone(), ctx.tx_serial.clone());
                module.add_button(Box::new(button));
            }
            modules.push(module);
        }
        Self { modules }
    }
}

impl PageTrait for ActPage {
    fn update(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, _status: &AppState) {
        ui.heading("Actuators Control");
        ui.add_space(10.0);

        for module in &mut self.modules {
            ui.group(|ui| {
                ui.colored_label(
                    Color32::from_rgb(200, 200, 255), 
                    egui::RichText::new(&module.name).size(16.0).strong()
                );
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal_wrapped(|ui| {
                    for button in &mut module.buttons {
                        button.render(ui);
                    }
                });
            });
            ui.add_space(15.0);
        }
    }
}