mod backup;
mod gui_backup;
mod mouse_controller;
mod eventi;
mod suoni;
mod cpu_usage;
mod dir_functions;

use std::env;
use std::path::{Path, PathBuf};
use egui::debug_text::print;
use serde::Deserialize;
use sysinfo::{DiskExt, System, SystemExt};
use gui_backup::ConfigApp;
use auto_launch::AutoLaunch;
use dir_functions::get_project_directory;
#[derive(Debug, Deserialize)]
struct ConfigData {
    backup_type: String,
    extension: Option<String>,
    input_path: String,
    output_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let exe_path_buf = std::env::current_exe()?;
    let exe_path = exe_path_buf
        .to_str()
        .ok_or("Failed to convert executable path to string")?;

    let auto = AutoLaunch::new("Backup",exe_path);

    if !auto.is_enabled()?{
        auto.enable()?;
        println!("Avvio automatico abilitato per il programma Backup.");
    }
    run_gui();
    let proj_dir = get_project_directory()?;
    let config_path = proj_dir.join("config.toml");
    println!("Config Path calcolato: {:?}", config_path);
    let (backup_type, extension, input_path, output_path) = load_config(&config_path)?;

    // 4. Verifica la validità dei percorsi
    verify_paths(&input_path, &output_path)?;

    mouse_controller::mouse_events(extension,&backup_type,&input_path,&output_path);

    Ok(())
}

fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Backup Configurator",
        options,
        Box::new(|_cc| Box::<ConfigApp>::default())
    ).expect("Errore nell'avvio della GUI");
}

fn load_config(path: &PathBuf) -> Result<(String, Option<String>, String, String), Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string(path)?;
    let config: ConfigData = toml::from_str(&config_str)?;
    Ok((config.backup_type, config.extension, config.input_path, config.output_path))
}

fn verify_paths(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if input_path.is_empty() || output_path.is_empty() {
        return Err("Please select both input and output paths.".into());
    }
    println!("Input Path: {:?}", input_path);
    println!("Output Path: {:?}", output_path);
    Ok(())
}