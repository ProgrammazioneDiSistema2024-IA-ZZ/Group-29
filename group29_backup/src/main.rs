mod eventi;
mod backup;
mod suoni;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut current_dir = env::current_dir().unwrap().parent().unwrap().to_path_buf();
    current_dir.push("config.toml");

    let config = backup::BackupConfig::load_from_file(current_dir.to_str().ok_or("Percorso non valido")?)?;

    // Imposta la destinazione del backup
    let destination = "C:/Users/Fabiano Vaglio/Desktop/destinazione";
    // Esegui il backup
    backup::perform_backup(&config, destination)?;

    Ok(())
}