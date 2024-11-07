mod backup;
mod gui_backup;
mod mouse_controller;
mod eventi;
mod suoni;
mod cpu_usage;

use std::env;
use std::path::{Path, PathBuf};
use egui::debug_text::print;
use serde::Deserialize;
use sysinfo::{DiskExt, System, SystemExt};
use gui_backup::ConfigApp;

#[derive(Debug, Deserialize)]
struct ConfigData {
    backup_type: String,
    extension: Option<String>,
    input_path: String,
    output_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Avvia la GUI e ottieni i percorsi di input e output
    run_gui();

    // 2. Ottieni la directory del progetto e il percorso del file di configurazione
    let config_path = get_project_directory()?;
    println!("Config Path: {:?}", config_path);
    // 3. Carica la configurazione dal file TOML
    let (backup_type, extension, input_path, output_path) = load_config(&config_path)?;

    // 4. Verifica la validità dei percorsi
    verify_paths(&input_path, &output_path)?;

    mouse_controller::mouse_events(extension,&backup_type,&input_path,&output_path);

    Ok(())
}

/// Funzione per avviare la GUI e ottenere i percorsi input/output
fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Backup Configurator",
        options,
        Box::new(|_cc| Box::<ConfigApp>::default())
    ).expect("Errore nell'avvio della GUI");
}

/// Funzione per ottenere la directory del progetto
fn get_project_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .ancestors()
        .nth(1)
        .ok_or("Impossibile ottenere la directory del progetto")?;
    let config_path = project_dir.join("group29_backup").join("config.toml");
    Ok(config_path)
}

/// Funzione per caricare la configurazione dal file TOML
fn load_config(path: &PathBuf) -> Result<(String, Option<String>, String, String), Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string(path)?;
    let config: ConfigData = toml::from_str(&config_str)?;
    Ok((config.backup_type, config.extension, config.input_path, config.output_path))
}

/// Funzione per verificare che entrambi i percorsi siano validi
fn verify_paths(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if input_path.is_empty() || output_path.is_empty() {
        return Err("Please select both input and output paths.".into());
    }
    println!("Input Path: {:?}", input_path);
    println!("Output Path: {:?}", output_path);
    Ok(())
}