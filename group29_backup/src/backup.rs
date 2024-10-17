use serde::{Deserialize};
use std::{env, fs};
use std::path::{PathBuf, Path};
use rfd::MessageDialog;
use crate::suoni::{play_sound_backup_ok, play_sound_backup_error};
use std::thread;
use sysinfo::{System, SystemExt, CpuExt};
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use rayon::prelude::*;

// Struttura per la configurazione del backup
#[derive(Debug, Deserialize)]
pub struct BackupConfig {
    backup_type: BackupType,      // Tipo di backup
}

#[derive(Debug, Deserialize)]
pub enum BackupType {
    FullFolder(bool),             // Backup completo della cartella
    FileType(String),             // Backup di un determinato tipo di file (es. ".txt")
}

impl BackupConfig {
    // Carica la configurazione da un file TOML
    pub fn load_from_file(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(config_path)?;
        let config: BackupConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
}

// Funzione per trovare la directory superiore (ad esempio, "Group-29")
fn find_project_root(start_path: &Path, project_name: &str) -> Option<PathBuf> {
    for ancestor in start_path.ancestors() {
        if ancestor.ends_with(project_name) {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

// Funzione per calcolare la dimensione totale dei file in un percorso
fn calculate_total_size(path: &PathBuf) -> u64 {
    let mut total_size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();
            total_size += metadata.len();
        }
    }
    total_size
}

// Funzione per loggare dimensione totale e tempo CPU su un file di log
fn log_backup_info(total_size: u64, cpu_usage: f32) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = "backup_log.txt";
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path)?;

    writeln!(
        file,
        "Backup completato:\nDimensione totale dei file: {} bytes\nUtilizzo medio della CPU: {:.2}%",
        total_size, cpu_usage
    )?;

    Ok(())
}

// Funzione per eseguire il backup
pub fn perform_backup(config: &BackupConfig, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?; // Ottiene la directory corrente

    // Trova la directory del progetto risalendo fino a "Group-29"
    let src_path = find_project_root(&current_dir, "Group-29")
        .ok_or("Impossibile trovare la directory del progetto Group-29")?;

    let dest_path = PathBuf::from(destination);

    println!("Esecuzione backup...");
    println!("Percorso sorgente: {:?}", src_path);
    println!("Percorso destinazione: {:?}", dest_path);

    if !src_path.exists() {
        println!("Il percorso sorgente non esiste!");
        play_sound_backup_error()?; // Riproduci il suono di errore
        return Err("Percorso sorgente non esiste".into());
    }

    if !src_path.is_dir() {
        println!("Il percorso sorgente non è una directory!");
        play_sound_backup_error()?; // Riproduci il suono di errore
        return Err("Il percorso sorgente non è una directory".into());
    }

    if !dest_path.exists() {
        println!("Il percorso destinazione non esiste!");
        play_sound_backup_error()?; // Riproduci il suono di errore
        return Err("Percorso destinazione non esiste".into());
    }

    if !dest_path.is_dir() {
        println!("Il percorso destinazione non è una directory!");
        play_sound_backup_error()?; // Riproduci il suono di errore
        return Err("Il percorso destinazione non è una directory".into());
    }

    // Inizia a misurare il tempo CPU
    let mut system = System::new_all();
    let start = Instant::now(); // Misura il tempo di inizio

    system.refresh_cpu();
    let start_cpu_usage = system.global_cpu_info().cpu_usage();

    // Crea la directory di destinazione se non esiste
    fs::create_dir_all(&dest_path)?;

    let mut total_size = 0; // Variabile per tracciare la dimensione totale dei file copiati

    println!("Inizio il processo di backup...");

    match &config.backup_type {
        BackupType::FullFolder(true) => {
            println!("Eseguendo un backup completo della cartella...");
            copy_directory_parallel(&src_path, &dest_path)?;  // Usa la funzione parallelizzata
            total_size = calculate_total_size(&src_path); // Calcola la dimensione dei file
        },
        BackupType::FileType(ext) => {
            println!("Eseguendo backup solo per file di tipo: {}", ext);

            for entry in fs::read_dir(&src_path)? {
                let entry = entry?;
                let path = entry.path();

                if let Some(file_ext) = path.extension() {
                    let file_ext_str = file_ext.to_str().unwrap_or("");

                    if file_ext_str == ext {
                        let dest_file = dest_path.join(entry.file_name());
                        println!("Copia del file: {:?} in {:?}", path, dest_file);

                        // Esegui la copia
                        if let Err(e) = fs::copy(&path, &dest_file) {
                            println!("Errore durante la copia del file {:?}: {:?}", path, e);
                            play_sound_backup_error()?; // Riproduci il suono di errore
                            return Err("Errore durante la copia del file".into());
                        } else {
                            total_size += entry.metadata()?.len();
                            println!("Copia riuscita di: {:?} in {:?}", path, dest_file);
                        }
                    }
                }
            }
            println!("Backup per file di tipo {} completato", ext);
        },
        _ => {
            println!("Nessun backup eseguito. Configurazione errata.");
            play_sound_backup_error()?; // Riproduci il suono di errore
            return Err("Configurazione backup errata".into());
        }
    }

    system.refresh_cpu();
    let end_cpu_usage = system.global_cpu_info().cpu_usage();
    let avg_cpu_usage = (start_cpu_usage + end_cpu_usage) / 2.0;
    let duration = start.elapsed();

    println!("Backup completato in: {:?}", duration);
    log_backup_info(total_size, avg_cpu_usage)?;

    let sound_thread = thread::spawn(|| {
        play_sound_backup_ok().unwrap();
    });

    MessageDialog::new()
        .set_title("Backup Completato")
        .set_description("Il backup è stato completato con successo.")
        .show();

    sound_thread.join().unwrap();

    Ok(())
}

// Funzione ricorsiva per copiare una directory in modo parallelo con controllo del numero di thread
fn copy_directory_parallel(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    // Crea la directory di destinazione se non esiste
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // Leggi tutte le voci nella directory sorgente
    let entries: Vec<_> = fs::read_dir(src)?
        .map(|entry| entry.unwrap())
        .collect();

    // Usa rayon per parallelizzare la copia dei file
    entries.par_iter().for_each(|entry| {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            // Se l'elemento è una directory, copia ricorsivamente
            copy_directory_parallel(&src_path, &dst_path).unwrap();
        } else {
            // Se l'elemento è un file, controlla se esiste già nella destinazione
            if dst_path.exists() {
                println!("Il file esiste già: {:?}, ignorato.", dst_path);
            } else {
                // Se non esiste, copia il file
                fs::copy(&src_path, &dst_path).unwrap();
                println!("Copia del file: {:?} in {:?}", src_path, dst_path);
            }
        }
    });

    Ok(())
}
