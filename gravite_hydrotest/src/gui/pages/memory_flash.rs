use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use probe_rs::probe::WireProtocol;
use probe_rs::probe::list::Lister;

use crate::gui::components::PageTrait;
use probe_rs::flashing::{DownloadOptions, FlashProgress};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use egui::Color32;

use crate::gui::components::AppState;
use crate::gui::components::PageContext;


pub enum FlashCommand {
    Start {path: PathBuf, chip: String}
}

pub enum FlashStatus {
    Idle,
    SearchProbe,
    Flashing(f32),
    Finished,
    Error(String),
}

pub fn spawn_flash_thread(
    rx_command: mpsc::Receiver<FlashCommand>,
    tx_status: mpsc::Sender<FlashStatus>,
    ctx: egui::Context,
) {
    thread::spawn(move || {
        while let Ok(FlashCommand::Start { path, chip }) = rx_command.recv() {
            let _ = tx_status.send(FlashStatus::SearchProbe);
            ctx.request_repaint();

            let tx = tx_status.clone();
            let ectx = ctx.clone();

            let result = || -> Result<(), Box<dyn std::error::Error>> {
                let lister = Lister::new();
                let probes = lister.list_all();
                let probe_info = probes.get(0).ok_or("Nie znaleziono programatora!")?;
                
                let mut probe = lister.open(probe_info)?;

                probe.select_protocol(WireProtocol::Swd)?;
                probe.set_speed(1000)?;

                let mut session = probe.attach_under_reset(chip, probe_rs::Permissions::default())?;
                
                let total_bytes = AtomicU64::new(0);
                let processed_bytes = AtomicU64::new(0);

                let progress = FlashProgress::new(move |info| {
                    use probe_rs::flashing::ProgressEvent;
                    
                    match info {
                        ProgressEvent::StartedProgramming { length } => {
                            total_bytes.store(length, Ordering::SeqCst);
                            processed_bytes.store(0, Ordering::SeqCst);
                        }
                        ProgressEvent::PageProgrammed { size, .. } => {
                            let current = processed_bytes.fetch_add(size as u64, Ordering::SeqCst) + size as u64;
                            let total = total_bytes.load(Ordering::SeqCst);
                            
                            if total > 0 {
                                let p = current as f32 / total as f32;
                                let _ = tx.send(FlashStatus::Flashing(p));
                                ectx.request_repaint();
                            }
                        }
                        _ => {}
                    }
                });

                let mut options = DownloadOptions::default();
                options.progress = Some(progress);

                probe_rs::flashing::download_file_with_options(
                    &mut session, 
                    &path, 
                    probe_rs::flashing::Format::Elf, 
                    options
                )?;

                let mut core = session.core(0)?;
                core.reset()?;

                Ok(())
            }();

            match result {
                Ok(_) => { let _ = tx_status.send(FlashStatus::Finished); }
                Err(e) => { 
                    eprintln!("Flash error: {:?}", e); 
                    let _ = tx_status.send(FlashStatus::Error(format!("{:?}", e))); 
                }
            }
            ctx.request_repaint();
        }
    });
}

pub struct FlashPage {
    tx_command: mpsc::Sender<FlashCommand>,
    selected_file: Option<PathBuf>,
    chip_name: String,
}

impl FlashPage {
    pub fn new(ctx: &PageContext) -> Self {
        Self {
            tx_command: ctx.tx_flash.clone(),
            selected_file: None,
            chip_name: "STM32F429ZIT6".to_string(),
        }
    }
}

impl PageTrait for FlashPage {
    fn update(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &AppState) {
        ui.heading("STM32 Firmware Uploader");

        ui.horizontal(|ui| {
            ui.label("Target Chip:");
            ui.text_edit_singleline(&mut self.chip_name);
        });

        ui.horizontal(|ui| {
            if ui.button("Pick ELF File").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Executable", &["elf"])
                    .pick_file() {
                    self.selected_file = Some(path);
                }
            }
            if let Some(path) = &self.selected_file {
                ui.label(format!("{}", path.file_name().unwrap().to_string_lossy()));
            }
        });

        ui.add_space(10.0);

        let is_busy = matches!(state.flash_status, FlashStatus::SearchProbe | FlashStatus::Flashing(_));
        let can_flash = self.selected_file.is_some() && !is_busy;

        if ui.add_enabled(can_flash, egui::Button::new("Flash memory")).clicked() {
            let _ = self.tx_command.send(FlashCommand::Start {
                path: self.selected_file.clone().unwrap(),
                chip: self.chip_name.clone(),
            });
        }

        ui.add_space(10.0);

        match &state.flash_status {
            FlashStatus::SearchProbe => { 
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Searching for ST-Link...");
                });
            }
            FlashStatus::Flashing(p) => { 
                ui.add(egui::ProgressBar::new(*p).text(format!("Flashing: {:.0}%", p * 100.0))); 
            }
            FlashStatus::Finished => { 
                ui.colored_label(Color32::GREEN, "Flash Successful!"); 
            }
            FlashStatus::Error(e) => { ui.colored_label(Color32::RED, format!("Error: {}", e)); }
            _ => { ui.label("Ready"); }
        }
    }
}