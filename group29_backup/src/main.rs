mod backup;
mod gui_backup;
mod mouse_controller;
mod eventi;
mod suoni;
use std::env;
use std::path::{PathBuf, Path};
use gui_backup::ConfigApp; // Import your GUI App

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the GUI application
    let app = ConfigApp::default();

    // Run the eframe application
    eframe::run_native("Backup Configurator", Default::default(), Box::new(|_cc| Box::<ConfigApp>::default()));



    mouse_controller::mouse_events();
    // Your existing backup logic can be called here if necessary
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    // Other code logic...

    Ok(())
}
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Ottieni il percorso della directory di lavoro corrente
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);

    // Ottieni la directory superiore (risalendo di un livello)
    let project_dir = current_dir
        .ancestors()
        .nth(1)  // Salta un solo livello per arrivare a Group-29
        .ok_or("Impossibile ottenere la directory del progetto")?;

    println!("Project directory: {:?}", project_dir);

    // Costruisci il percorso del file di configurazione
    let config_path = project_dir.join("config.toml");
    println!("Config path: {:?}", config_path);

    // Carica il file di configurazione
    let config_str = config_path.to_str().ok_or("Percorso non valido")?;
    let config = backup::BackupConfig::load_from_file(config_str)?;

    // Specifica il percorso della chiavetta USB o di una cartella specifica
    let usb_destination = PathBuf::from("C:/Users/Fabiano Vaglio/Desktop/destinazione");

    // Esegui il backup
    let usb_str = usb_destination.to_str().ok_or("Percorso non valido")?;
    backup::perform_backup(&config, usb_str)?;



    Ok(())
}*/


