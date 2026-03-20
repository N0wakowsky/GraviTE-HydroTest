use std::path::Path;

use config::AppConfig;

mod app;
mod gui;
mod config;
mod comm;

fn main() -> eframe::Result<()> {
    let config = AppConfig::load(Path::new("config.yaml")).unwrap_or_else(|e| {
        eprintln!("Configuration error: {}", e);
        AppConfig::default()
    });


    let options = eframe::NativeOptions::default();
    eframe::run_native("GraviTE Control Panel", options, Box::new(move |_cc| Ok(
        Box::new(app::App::new(config))
    )))
}