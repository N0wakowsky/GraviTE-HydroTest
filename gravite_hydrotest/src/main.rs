// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, path::PathBuf};

use eframe::NativeOptions;
use egui::Vec2;

use crate::{app::App};

mod gui;
mod config;
mod app;

fn get_path() -> PathBuf {
    let mut exe_path = env::current_exe().expect("Failed to get exe path");

    exe_path.pop();
    exe_path
}

fn main() -> Result<(), eframe::Error> {
    let base = get_path();
    let config_path = base.join("config.yaml");
    let procedure_dir = base.join("procedures");


    let mut config = config::AppConfig::new();
    config.load_act_config(&config_path).expect("Config.yaml load error");
    config.load_procedure_config(&procedure_dir).expect("Procedures loading failed");

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1024.0, 768.0))
            .with_min_inner_size(Vec2::new(800.0, 600.0))
            .with_title("GraviTE HydroTest"),
        ..Default::default()
    };

    eframe::run_native(
        "GraviTE", 
        options, 
        Box::new(|cc| Ok(Box::new(App::new(config, cc.egui_ctx.clone())))),
    )
}