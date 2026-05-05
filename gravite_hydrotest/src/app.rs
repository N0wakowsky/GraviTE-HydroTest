use std::collections::HashMap;
use std::sync::mpsc;
use crate::gui::components::AppState;

use crate::gui::pages::serial_control::SerialStatus;
use crate::gui::pages::memory_flash::FlashStatus;
use crate::gui::pages::proc_control::ProcedureStatus;

use crate::{config::{ActuatorsRegister, AppConfig}, gui::{components::{PageFactory, PageTrait, PageType}}};

use crate::gui::pages::serial_control::spawn_serial_thread;
use crate::gui::pages::memory_flash::spawn_flash_thread;
use crate::gui::pages::proc_control::spawn_proc_thread;

use crate::gui::components::PageContext;


pub struct App {
    current_page_type: PageType,
    pages: HashMap<PageType, Box<dyn PageTrait>>,
    state: AppState,
    
    rx_ser_stat: mpsc::Receiver<SerialStatus>,
    rx_fl_stat: mpsc::Receiver<FlashStatus>,
    rx_proc_stat: mpsc::Receiver<ProcedureStatus>,
}

impl App {
    pub fn new(config: AppConfig, ctx: egui::Context) -> Self {
        let act_register = ActuatorsRegister::from_config(&config);

        let (tx_ser_cmd, rx_ser_cmd) = mpsc::channel();
        let (tx_ser_stat, rx_ser_stat) = mpsc::channel();
        spawn_serial_thread(rx_ser_cmd, tx_ser_stat, act_register.clone(), ctx.clone());

        let (tx_fl_cmd, rx_fl_cmd) = mpsc::channel();
        let (tx_fl_stat, rx_fl_stat) = mpsc::channel();
        spawn_flash_thread(rx_fl_cmd, tx_fl_stat, ctx.clone());

        let (tx_proc_cmd, rx_proc_cmd) = mpsc::channel();
        let (tx_proc_stat, rx_proc_stat) = mpsc::channel();
        spawn_proc_thread(rx_proc_cmd, tx_proc_stat, tx_ser_cmd.clone(), config.clone(), ctx.clone(), act_register.clone());

        let state = AppState {
            serial_status: SerialStatus::Disconnected,
            flash_status: FlashStatus::Idle,
            proc_state: ProcedureStatus::Idle,
            available_ports: vec![],
        };

        let page_ctx = PageContext {
            config: config.clone(),
            act_register,
            tx_serial: tx_ser_cmd,
            tx_flash: tx_fl_cmd,
            tx_proc: tx_proc_cmd,
        };

        let act_page = PageFactory::create(PageType::Actuators, &page_ctx);
        let serial_page = PageFactory::create(PageType::SerialPort, &page_ctx);
        let flash_page = PageFactory::create(PageType::Flash, &page_ctx);
        let proc_page = PageFactory::create(PageType::Procedures, &page_ctx);

        let mut pages: HashMap<PageType, Box<dyn PageTrait>> = HashMap::new();
        pages.insert(PageType::Actuators, act_page);
        pages.insert(PageType::SerialPort,  serial_page);
        pages.insert(PageType::Flash, flash_page);
        pages.insert(PageType::Procedures, proc_page);

        Self {
            current_page_type: PageType::Actuators,
            pages,
            state,
            rx_ser_stat,
            rx_fl_stat,
            rx_proc_stat,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.rx_ser_stat.try_recv() {
            match msg {
                SerialStatus::PortList(p) => self.state.available_ports = p,
                status => self.state.serial_status = status,
            }
        }
        while let Ok(status) = self.rx_fl_stat.try_recv() { self.state.flash_status = status; }
        while let Ok(status) = self.rx_proc_stat.try_recv() { self.state.proc_state = status; }

        egui::SidePanel::left("menu").show(ctx, |ui| {
            if ui.selectable_label(self.current_page_type == PageType::Actuators, "Actuators").clicked() {
                self.current_page_type = PageType::Actuators;
            }
            if ui.selectable_label(self.current_page_type == PageType::Procedures, "Procedures").clicked() {
                self.current_page_type = PageType::Procedures;
            }
            if ui.selectable_label(self.current_page_type == PageType::SerialPort, "Serial Port").clicked() {
                self.current_page_type = PageType::SerialPort;
            }
            if ui.selectable_label(self.current_page_type == PageType::Flash, "Flash").clicked() {
                self.current_page_type = PageType::Flash;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(page) = self.pages.get_mut(&self.current_page_type) {
                page.update(ctx, ui, &self.state);
            }
        });
    }
}