mod eventi;
mod backup;
mod suoni;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ottieni il percorso della directory di lavoro corrente
    let current_dir = env::current_dir()?;

    // Ottieni la directory superiore (rimuovi la parte 'target')
    let project_dir = current_dir.parent()
        .ok_or("Impossibile ottenere la directory superiore")?
        .parent()
        .ok_or("Impossibile ottenere la directory del progetto")?;

    // Costruisci il percorso del file di configurazione
    let config_path = project_dir.join("config.toml");
    let config = backup::BackupConfig::load_from_file(config_path.to_str().ok_or("Percorso non valido")?)?;

    // Specifica il percorso della chiavetta USB o di una cartella specifica
    let usb_destination = PathBuf::from("C:/Users/sagli/Desktop/uni/PROGRAMMAZIONE DI SISTEMA/SECONDA PARTE/Progetto/destinazione"); // Cambia "D:/" con la lettera della tua chiavetta USB

    // Esegui il backup
    backup::perform_backup(&config, usb_destination.to_str().ok_or("Percorso non valido")?)?;

    Ok(())
}
