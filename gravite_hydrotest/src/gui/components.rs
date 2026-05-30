//! Uniwersalne komponenty, interfejsy widoków i zarządzanie stanem GUI.
//!
//! Definiuje podstawowe traity dla elementów interfejsu
//! oraz wzorzec Factory pozwalający na łatwe dodawanie nowych podstron w programie.

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


/// Interfejs dla niestandardowych przycisków w interfejsie egui.
pub trait ButtonTrait {
    /// Zwraca etykietę wyświetlaną na przycisku.
    fn label(&self) -> &str;
    /// Definiuje bieżący kolor wypełnienia przycisku, zależny np. od jego logicznego stanu.
    fn color(&self, ui: &egui::Ui) -> Color32;
    /// Zwraca minimalny preferowany rozmiar (szerokość, wysokość) elementu na ekranie.
    fn size(&self) -> Vec2;
    /// Logika wywoływana w momencie kliknięcia przycisku.  
    fn on_click(&mut self);

    /// Domyślna implementacja renderowania i obsługi kliknięcia w oknie egui.
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

/// Wspólny interfejs dla każdej podstrony aplikacji.
pub trait PageTrait {
    /// Główna metoda renderująca UI danej zakładki. Wywoływana przez App::update.
    fn update(&mut self, ctx: &Context, ui: &mut Ui, state: &AppState);
}

/// Fabryka komponentów UI budująca zakładki według wariantu PageType.
pub struct PageFactory;

impl PageFactory {
    /// Tworzy nową instancję strony implementującą trait PageTrait 
    /// w oparciu o podany kontekst strony (konfiguracja i kanały MPSC).
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