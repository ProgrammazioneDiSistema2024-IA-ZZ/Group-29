//#![windows_subsystem = "windows"]
mod backup;
mod gui_backup;
mod mouse_controller;
mod suoni;
mod cpu_usage;
mod dir_functions;
mod tracker;
mod eventi_pulito;
mod config_autorun;

use std::thread;
use std::process::Command;
use dir_functions::get_project_directory;
use crate::config_autorun::{configure_autorun, verify_paths, load_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Avvia la registrazione dell'utilizzo della CPU in un thread separato
    thread::spawn(|| {
        cpu_usage::log_cpu_usage(); // Funzione che continua a loggare la CPU ogni 10 secondi
    });

    // Configura l'avvio automatico su Windows
    configure_autorun()?;

    // Avvia la GUI come processo separato
    println!("Avvio della GUI come processo separato...");
    if let Err(e) = Command::new("gui_backup")
        .spawn() // Avvia la GUI senza bloccare il thread principale
    {
        eprintln!("Errore nell'avvio della GUI: {}", e);
    }

    // Configurazione e verifica dei percorsi
    let proj_dir = get_project_directory()?;
    let config_path = proj_dir.join("config.toml");
    let (backup_type, extension, input_path, output_path) = load_config(&config_path)?;

    verify_paths(&input_path, &output_path)?;

    // Avvia il controller del mouse
    mouse_controller::mouse_events(extension, &backup_type, &input_path, &output_path);

    Ok(())
}
