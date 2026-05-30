//! # GraviTE HydroTest
//! 
//! Oficjalna dokumentacja techniczna aplikacji **GraviTE HydroTest**.

//! Punkt uruchomienia aplikacji GraviTE HydroTest.
//!
//! Moduł odpowiada za załadowanie konfiguracji z plików YAML
//! oraz uruchomienie głównego okna aplikacji.


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, path::PathBuf};

use eframe::NativeOptions;
use egui::Vec2;

use crate::{app::App};

mod gui;
mod config;
mod app;

/// Pobiera bezwzględną ścieżkę do katalogu, w którym znajduje się plik wykonywalny aplikacji.
///
/// # Panics
/// Funkcja panikuje, jeśli system operacyjny nie zwróci ścieżki do aktualnie wykonywanego procesu.
fn get_path() -> PathBuf {
    let mut exe_path = env::current_exe().expect("Failed to get exe path");

    exe_path.pop();
    exe_path
}


/// Główna funkcja programu uruchamiająca interfejs graficzny.
///
/// Ładuje plik `config.yaml` oraz procedury z folderu `procedures`, a następnie
/// inicjalizuje środowisko `eframe` i przekazuje kontrolę do struktury `App`.
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