use serde::{Deserialize};
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use rfd::MessageDialog;
use std::ffi::OsStr;

// Struttura per la configurazione del backup
#[derive(Debug, Deserialize)]
pub struct BackupConfig {
    source_path: String,          // Percorso sorgente del backup
    backup_type: BackupType,      // Tipo di backup
}

#[derive(Debug, Deserialize)]
pub enum BackupType {
    FullFolder,                   // Backup completo della cartella
    FileType(String),              // Backup di un determinato tipo di file (es. ".txt")
}

impl BackupConfig {
    // Carica la configurazione da un file TOML
    pub fn load_from_file(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(config_path)?;
        let config: BackupConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}

// Funzione per eseguire il backup
pub fn perform_backup(config: &BackupConfig, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src_path = PathBuf::from(&config.source_path);
    let dest_path = PathBuf::from(destination);

    match config.backup_type {
        BackupType::FullFolder => {
            // Esegue il backup completo della cartella
            fs::create_dir_all(&dest_path)?;
            let output = Command::new("cp")
                .arg("-r")
                .arg(&src_path)
                .arg(&dest_path)
                .output()?;
            println!("Backup completato: {:?}", output);
        },
        BackupType::FileType(ref ext) => {
            // Esegue il backup solo dei file di un determinato tipo
            for entry in fs::read_dir(&src_path)? {
                let entry = entry?;
                let path = entry.path();

                // Ottieni l'estensione del file e confrontala con `ext`
                if let Some(file_ext) = path.extension() {
                    if let Some(file_ext_str) = file_ext.to_str() {
                        if file_ext_str == ext {
                            let dest_file = dest_path.join(entry.file_name());
                            fs::copy(&path, dest_file)?;
                        }
                    }
                }
            }
            println!("Backup per file {} completato", ext);
        },
    }
    // Mostra una finestra di conferma
    MessageDialog::new()
        .set_title("Backup Completato")
        .set_description("Il backup è stato completato con successo.")
        .show();

    Ok(())
}
