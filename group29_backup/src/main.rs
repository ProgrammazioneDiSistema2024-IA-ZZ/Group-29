mod eventi;
mod backup;
mod suoni;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = backup::BackupConfig::load_from_file("C:/Users/sagli/Desktop/uni/PROGRAMMAZIONE DI SISTEMA/SECONDA PARTE/Group-29/config.toml")?;

    // Imposta la destinazione del backup
    let destination = "C:/Users/sagli/Desktop/uni/PROGRAMMAZIONE DI SISTEMA/SECONDA PARTE/Progetto/destinazione";

    // Esegui il backup
    backup::perform_backup(&config, destination)?;

    Ok(())
}