use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Write;
#[cfg(target_os = "macos")]
use tauri; // Usa Tauri per macOS
#[cfg(not(target_os = "macos"))]
use eframe; // Usa eframe per Windows e Linux
use gui_backup::ConfigApp;
use serde::Deserialize;
#[cfg(target_os="windows")]
use winreg::enums::*;
use winreg::RegKey;
use crate::gui_backup;

#[derive(Debug, Deserialize)]
struct ConfigData {
    backup_type: String,
    extension: Option<String>,
    input_path: String,
    output_path: String,
}

pub fn configure_autorun() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path_buf = env::current_exe()?; // Ottieni il percorso dell'eseguibile
    let exe_path = exe_path_buf
        .to_str()
        .ok_or("Failed to convert executable path to string")?;
    let exe_path_with_quotes = format!("\"{}\"", exe_path); // Aggiungi virgolette

    #[cfg(target_os = "windows")]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;

        let app_name = "Backup";
        match run.get_value::<String, _>(app_name) {
            Ok(existing_path) if existing_path == exe_path_with_quotes => {
                println!("Avvio automatico già configurato su Windows.");
            }
            _ => {
                println!("Configurazione dell'avvio automatico su Windows...");
                run.set_value(app_name, &exe_path_with_quotes)?;
                println!("Avvio automatico configurato con successo su Windows.");
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let autostart_dir = dirs::config_dir()
            .ok_or("Impossibile trovare la directory di configurazione")?
            .join("autostart");

        fs::create_dir_all(&autostart_dir)?;
        let desktop_entry_path = autostart_dir.join("backup.desktop");
        let mut desktop_file = File::create(desktop_entry_path)?;

        let desktop_content = format!(
            "[Desktop Entry]
            Type=Application
            Name=Backup
            Exec={}
            X-GNOME-Autostart-enabled=true
            ",
            exe_path
        );

        desktop_file.write_all(desktop_content.as_bytes())?;
        println!("Avvio automatico configurato con successo su Linux.");
    }

    #[cfg(target_os = "macos")]
    {
        let launch_agents_dir = dirs::home_dir()
            .ok_or("Impossibile trovare la directory home")?
            .join("Library/LaunchAgents");

        fs::create_dir_all(&launch_agents_dir)?;
        let plist_path = launch_agents_dir.join("com.example.backup.plist");
        let mut plist_file = File::create(plist_path)?;

        let plist_content = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
        <plist version=\"1.0\">
        <dict>
            <key>Label</key>
            <string>com.example.backup</string>
            <key>ProgramArguments</key>
            <array>
                <string>{}</string>
            </array>
            <key>RunAtLoad</key>
            <true/>
        </dict>
        </plist>",
            exe_path
        );

        plist_file.write_all(plist_content.as_bytes())?;
        println!("Avvio automatico configurato con successo su macOS.");
    }

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn run_gui() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![/* Comandi personalizzati */])
        .run(tauri::generate_context!())
        .expect("Errore nell'avvio dell'interfaccia grafica con Tauri");
}

#[cfg(not(target_os = "macos"))]
pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Backup Configurator",
        options,
        Box::new(|_cc| {
            
            Box::new(ConfigApp::default()) as Box<dyn eframe::App>
        }),
    ).expect("Errore nell'avvio della GUI");
}


pub fn load_config(path: &PathBuf) -> Result<(String, Option<String>, String, String), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    let path = {
        let config_dir = dirs::data_local_dir()
            .ok_or("Impossibile trovare la directory dei dati locali")?
            .join("Backup");

        std::fs::create_dir_all(&config_dir)?;
        config_dir.join("config.toml")
    };

    let config_str = std::fs::read_to_string(&path)?;
    let config: ConfigData = toml::from_str(&config_str)?;

    Ok((config.backup_type, config.extension, config.input_path, config.output_path))
}

pub fn verify_paths(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if input_path.is_empty() || output_path.is_empty() {
        return Err("Please select both input and output paths.".into());
    }
    println!("Input Path: {:?}", input_path);
    println!("Output Path: {:?}", output_path);
    Ok(())
}
