use std::path::Path;

mod config;

mod app;
mod gui;
mod comm;

fn main() -> Result<(), eframe::Error>{
    let mut config = config::AppConfig::new();
    config.load_act_config(Path::new("config.yaml")).expect("Config.yaml load error");
    config.load_procedure_config(Path::new("procedures")).expect("Procedures loading failed");

    let options = eframe::NativeOptions::default();
    eframe::run_native("GraviTE Control Panel", options, Box::new(move |_cc| Ok(
        Box::new(app::App::new(config))
    )))
}
