mod eventi;
mod backup;
mod suoni;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = backup::BackupConfig::load_from_file("C:/Users/Fabiano Vaglio/RustroverProjects/Group-29/config.toml")?;

    // Imposta la destinazione del backup
    let destination = "C:/Users/Fabiano Vaglio/Desktop/destinazione";
    // Esegui il backup
    backup::perform_backup(&config, destination)?;

    Ok(())
}