mod eventi;
mod backup;
mod suoni;
mod cpu_usage;

use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = backup::BackupConfig::load_from_file("C:/Users/sagli/Desktop/uni/PROGRAMMAZIONE DI SISTEMA/SECONDA PARTE/Group-29/config.toml")?;

    let destination = "C:/Users/sagli/Desktop/uni/PROGRAMMAZIONE DI SISTEMA/SECONDA PARTE/Progetto/destinazione";

    // Avvia il monitoraggio del consumo di CPU in un thread separato
    let cpu_log_thread = thread::spawn(|| {
        cpu_usage::log_cpu_usage();
    });

    // Esegui il backup
    backup::perform_backup(&config, destination)?;

    // Aspetta che il thread del log CPU termini (se necessario)
    cpu_log_thread.join().expect("Errore nel thread del log CPU");

    Ok(())
}