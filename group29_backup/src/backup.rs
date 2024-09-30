use serde::{Deserialize};
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use native_dialog::MessageDialog;
use std::ffi::OsStr;
use crate::suoni::play_sound_backup_ok;

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

    println!("Esecuzione backup...");
    println!("Percorso sorgente: {:?}", src_path);
    println!("Percorso destinazione: {:?}", dest_path);

    if !src_path.exists() {
        println!("Il percorso sorgente non esiste!");
        return Err("Percorso sorgente non esiste".into());
    }

    // Controllo se è una directory
    if !src_path.is_dir() {
        println!("Il percorso sorgente non è una directory!");
        return Err("Il percorso sorgente non è una directory".into());
    }

    // Crea la directory di destinazione se non esiste
    fs::create_dir_all(&dest_path)?;

    println!("Inizio il processo di backup...");

    match config.backup_type {
        BackupType::FullFolder => {
            println!("Eseguendo un backup completo della cartella...");
            let output = Command::new("cp")
                .arg("-r")
                .arg(&src_path)
                .arg(&dest_path)
                .output()?;

            if !output.status.success() {
                println!("Errore durante la copia della cartella: {:?}", output);
                return Err("Errore durante il backup della cartella completa".into());
            }

            println!("Backup della cartella completato: {:?}", output);
        },
        BackupType::FileType(ref ext) => {
            println!("Eseguendo backup solo per file di tipo: {}", ext);

            let entries: Vec<_> = fs::read_dir(&src_path)?.collect();


            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if let Some(file_ext) = path.extension() {
                    let file_ext_str = file_ext.to_str().unwrap_or("");

                    // Stampa il nome e l'estensione del file
                    println!("File trovato: {:?}", path.file_name());
                    println!("Estensione file: {:?}", file_ext_str);

                    if file_ext_str == ext {
                        let dest_file = dest_path.join(entry.file_name());
                        println!("Copia del file: {:?} in {:?}", path, dest_file);

                        // Esegui la copia
                        if let Err(e) = fs::copy(&path, &dest_file) {
                            println!("Errore durante la copia del file {:?}: {:?}", path, e);
                            return Err("Errore durante la copia del file".into());
                        } else {
                            // Stampa un messaggio di conferma per la copia riuscita
                            println!("Copia riuscita di: {:?} in {:?}", path, dest_file);
                        }
                    } else {
                        println!("Estensione non corrisponde. Ignoro il file: {:?}", path.file_name());
                    }
                }
            }
            println!("Backup per file {} completato", ext);
        },
    }

    MessageDialog::new()
        .set_title("Backup Completato")
        .set_text("Il backup è stato completato con successo.")
        .show_alert()
        .unwrap();


    Ok(())
}
