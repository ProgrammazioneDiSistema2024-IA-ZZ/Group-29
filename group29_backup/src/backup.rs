use serde::{Deserialize};
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use rfd::MessageDialog;
use std::ffi::OsStr;
use crate::suoni::{play_sound_backup_ok, play_sound_backup_error};
use std::thread;
use sysinfo::{System, SystemExt, CpuExt}; // Per raccogliere informazioni sul sistema e CPU
use std::time::Instant;
use std::fs::OpenOptions;
use std::io;

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

// Funzione per calcolare la dimensione totale dei file in un percorso
fn calculate_total_size(path: &PathBuf) -> u64 {
    let mut total_size = 0;

    // Itera sui file per calcolare la dimensione totale
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
        "Backup completato:\nDimensione totale dei file: {} bytes\nTempo CPU utilizzato: {:.2}%",
        total_size, cpu_usage
    )?;

    Ok(())
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

    // Crea la directory di destinazione se non esiste
    fs::create_dir_all(&dest_path)?;

    let mut total_size = 0; // Variabile per tracciare la dimensione totale dei file copiati

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
                play_sound_backup_error()?; // Riproduci il suono di errore
                return Err("Errore durante il backup della cartella completa".into());
            }

            total_size = calculate_total_size(&src_path); // Calcola la dimensione dei file

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

                    if file_ext_str == ext {
                        let dest_file = dest_path.join(entry.file_name());
                        println!("Copia del file: {:?} in {:?}", path, dest_file);

                        // Esegui la copia
                        if let Err(e) = fs::copy(&path, &dest_file) {
                            println!("Errore durante la copia del file {:?}: {:?}", path, e);
                            play_sound_backup_error()?; // Riproduci il suono di errore
                            return Err("Errore durante la copia del file".into());
                        } else {
                            // Calcola la dimensione del file copiato
                            total_size += entry.metadata()?.len();
                            println!("Copia riuscita di: {:?} in {:?}", path, dest_file);
                        }
                    } else {
                        println!("Estensione non corrisponde. Ignoro il file: {:?}", path.file_name());
                    }
                }
            }
            println!("Backup per file {} completato", ext);
        },
        // Caso di fallback, se FullFolder è false non fa nulla
        _ => {
            println!("Nessun backup eseguito. Configurazione errata.");
            play_sound_backup_error()?; // Riproduci il suono di errore
            return Err("Configurazione backup errata".into());
        }
    }

    // Calcola il tempo CPU e totale impiegato
    system.refresh_cpu();
    let cpu_usage = system.global_cpu_info().cpu_usage(); // Ottieni il consumo globale di CPU
    let duration = start.elapsed(); // Ottieni il tempo totale impiegato

    println!("Backup completato in: {:?}", duration);

    // Crea il log con le informazioni del backup
    log_backup_info(total_size, cpu_usage)?;

    // Crea un nuovo thread per riprodurre il suono di successo
    let sound_thread = thread::spawn(|| {
        play_sound_backup_ok().unwrap(); // Gestisci eventuali errori di riproduzione
    });

    // Mostra il messaggio di completamento
    MessageDialog::new()
        .set_title("Backup Completato")
        .set_description("Il backup è stato completato con successo.")
        .show();

    // Aspetta che il thread del suono finisca
    sound_thread.join().unwrap();

    Ok(())
}

// Funzione ricorsiva per copiare una directory
fn copy_directory(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
// Crea la directory di destinazione se non esiste
    if !dst.exists() {
    fs::create_dir_all(dst)?;
    }
    // Leggi tutte le voci nella directory sorgente
    for entry in fs::read_dir(src)? {
    let entry = entry?;
    let src_path = entry.path();
    let dst_path = dst.join(entry.file_name());
    if src_path.is_dir() {
    // Se l'elemento è una directory, copia ricorsivamente
    copy_directory(&src_path, &dst_path)?;
    }
    else {            // Se l'elemento è un file, copialo
    fs::copy(&src_path, &dst_path)?;
    println!("Copia del file: {:?} in {:?}", src_path, dst_path);
    }
    }
Ok(())
}
