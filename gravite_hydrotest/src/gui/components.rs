use std::sync::mpsc;

use egui::{Color32, Context, Ui, Vec2};

use crate::config::{ActuatorsRegister, AppConfig};

use crate::gui::pages::act_control::{ActPage};
use crate::gui::pages::proc_control::ProcPage;
use crate::gui::pages::serial_control::{SerialCommand, SerialPage, SerialStatus};
use crate::gui::pages::memory_flash::FlashPage;

use crate::gui::pages::memory_flash::{FlashCommand, FlashStatus};
use crate::gui::pages::proc_control::ProcedureCommand;
use crate::gui::pages::proc_control::ProcedureStatus;


pub trait ButtonTrait {
    fn label(&self) -> &str;
    fn color(&self, ui: &egui::Ui) -> Color32;
    fn size(&self) -> Vec2;
    fn on_click(&mut self);

    fn render(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let button = egui::Button::new(self.label())
            .fill(self.color(ui))
            .min_size(self.size());
        
        let response = ui.add(button);
        
        if response.clicked() {
            self.on_click();
        }
        
        response
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageType {
    Actuators,
    Procedures,
    SerialPort,
    Flash,
}

pub struct AppState {
    pub serial_status: SerialStatus,
    pub flash_status: FlashStatus,
    pub proc_state: ProcedureStatus,
    pub available_ports: Vec<String>,
}

pub struct PageContext {
    pub config: AppConfig,
    pub act_register: ActuatorsRegister,
    pub tx_serial: mpsc::Sender<SerialCommand>,
    pub tx_flash: mpsc::Sender<FlashCommand>,
    pub tx_proc: mpsc::Sender<ProcedureCommand>,
}


pub trait PageTrait {
    fn update(&mut self, ctx: &Context, ui: &mut Ui, state: &AppState);
}

// factory
pub struct PageFactory;

impl PageFactory {
    pub fn create(
        page_type: PageType, 
        ctx: &PageContext
    ) -> Box<dyn PageTrait> {
        match page_type {
            PageType::Actuators => Box::new(ActPage::new(
                ctx
            )),
            PageType::Procedures => Box::new(ProcPage::new(
                ctx
            )),
            PageType::SerialPort => Box::new(SerialPage::new(
                ctx
            )),
            PageType::Flash => Box::new(FlashPage::new(
                ctx
            )),
        }
    }
}