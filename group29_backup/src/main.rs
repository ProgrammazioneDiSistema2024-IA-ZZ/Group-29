mod backup;
mod gui_backup;
mod mouse_controller;
mod eventi;
mod suoni;
mod cpu_usage;
mod dir_functions;

use std::{env, thread};
use std::path::{Path, PathBuf};
use serde::Deserialize;
use gui_backup::ConfigApp;
use winreg::enums::*;
use winreg::RegKey;
use dir_functions::get_project_directory;
use crate::cpu_usage::log_cpu_usage;

#[derive(Debug, Deserialize)]
struct ConfigData {
    backup_type: String,
    extension: Option<String>,
    input_path: String,
    output_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Avvia la registrazione dell'utilizzo della CPU in un thread separato
    thread::spawn(|| {
        log_cpu_usage(); // Funzione che continua a loggare la CPU ogni 10 secondi
    });

    // Configura l'avvio automatico su Windows
    configure_autorun()?;

    // Avvia l'interfaccia grafica
    run_gui();

    // Configurazione e verifica dei percorsi
    let proj_dir = get_project_directory()?;
    let config_path = proj_dir.join("config.toml");
    println!("Config Path calcolato: {:?}", config_path);
    let (backup_type, extension, input_path, output_path) = load_config(&config_path)?;

    verify_paths(&input_path, &output_path)?;

    // Avvia il controller del mouse
    mouse_controller::mouse_events(extension, &backup_type, &input_path, &output_path);

    Ok(())
}

fn configure_autorun() -> Result<(), Box<dyn std::error::Error>> {
    // Ottieni il percorso dell'eseguibile corrente come PathBuf
    let exe_path_buf = env::current_exe()?; // PathBuf è ora una variabile persistente

    // Get the absolute path as a string
    let exe_path = exe_path_buf
        .to_str()
        .ok_or("Failed to convert executable path to string")?;

    // Check if the path starts with ".\" and remove it if present
    let exe_path = if exe_path.starts_with(".\\") {
        &exe_path[2..] // Remove the ".\" prefix
    } else {
        exe_path
    };

    // Add quotes to the path
    let exe_path_with_quotes = format!("\"{}\"", exe_path);

    // Log for debugging purposes
    println!("Percorso dell'eseguibile con virgolette: {}", exe_path_with_quotes);

    // Accedi al registro di sistema
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;

    // Controlla se già esiste una chiave per l'applicazione
    let app_name = "Backup";
    match run.get_value::<String, _>(app_name) {
        Ok(existing_path) if existing_path == exe_path_with_quotes => {
            println!("Avvio automatico già configurato.");
        }
        _ => {
            // Imposta il percorso dell'applicazione per l'avvio automatico
            println!("Configurazione dell'avvio automatico...");
            run.set_value(app_name, &exe_path_with_quotes)?; // Scrivi il percorso con le virgolette
            println!("Avvio automatico configurato con successo.");
        }
    }

    Ok(())
}

fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Backup Configurator",
        options,
        Box::new(|_cc| Box::<ConfigApp>::default()),
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