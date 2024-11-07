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
pub fn perform_backup(backup_type: &str, extension: Option<&str>, src_path: &PathBuf, dest_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !src_path.exists() {
        play_sound_backup_error()?;
        return Err("Percorso sorgente non esiste".into());
    }

    if (!src_path.is_dir() || !dest_path.exists() ||  !dest_path.is_dir()){
        play_sound_backup_error()?;
        return Err("Percorso sorgente o destinazione non valido".into());
    }

    let mut system = System::new_all();
    let start = Instant::now();
    system.refresh_cpu();
    let start_cpu_usage = system.global_cpu_info().cpu_usage();

    fs::create_dir_all(&dest_path)?;

    let mut total_size = 0;
    print!("backup type: {}", backup_type);
    match backup_type {
        "FullFolder" => {
            copy_directory_parallel(&src_path, &dest_path)?;
            total_size = calculate_total_size(&src_path);
        },
        "FileType" => {
            if let Some(ext) = extension {
                println!("Eseguendo backup solo per file di tipo: {}", ext);
                for entry in fs::read_dir(&src_path)? {
                    let entry = entry?;
                    let path = entry.path();

                    if let Some(file_ext) = path.extension() {
                        if file_ext == ext {
                            let dest_file = dest_path.join(entry.file_name());
                            fs::copy(&path, &dest_file)?;
                            total_size += entry.metadata()?.len();
                        }
                    }
                }
            } else {
                return Err("Estensione file non specificata per FileType".into());
            }
        },
        _ => return Err("Tipo di backup non valido".into()),
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