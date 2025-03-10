#![windows_subsystem = "windows"]
mod backup;
mod gui_backup;
mod mouse_controller;
mod suoni;
mod cpu_usage;
mod dir_functions;
mod tracker;
mod eventi;
mod config_autorun;

use std::{path::PathBuf, thread};
use std::process::Command;
use dir_functions::get_project_directory;
use crate::config_autorun::{configure_autorun, verify_paths, load_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    thread::spawn(|| {
        cpu_usage::log_cpu_usage();
    });

    configure_autorun()?;

    #[cfg(target_os = "windows")]
    const GUI_PATH: &str = "gui_backup.exe";

    #[cfg(any(target_os = "linux"))]
    const GUI_PATH: &str = "./gui_backup";

    let proj_dir_maco = get_project_directory()?;
    let config_path_maco = proj_dir_maco.join("./release/macos/gui_backup");
    #[cfg(any(target_os = "macos"))]
    let  GUI_PATH: &str = config_path_maco.to_str().unwrap();

    println!("Current directory: {:?}", std::env::current_dir()?);

    // Avvia la GUI come processo separato
    println!("Avvio della GUI come processo separato...");
    match Command::new(GUI_PATH).spawn() {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                eprintln!("Errore durante l'esecuzione della GUI: {}", e);
            }
        }
        Err(e) => {
            println!("{}",GUI_PATH);
            eprintln!("Errore nell'avvio della GUI: {}", e);
        }
    }

    let proj_dir = get_project_directory()?;
    let config_path = proj_dir.join("config.toml");
    let (backup_type, extension, input_path, output_path) = load_config(&config_path)?;

    verify_paths(&input_path, &output_path)?;

    mouse_controller::mouse_events(extension, &backup_type, &input_path, &output_path);

    Ok(())
}
