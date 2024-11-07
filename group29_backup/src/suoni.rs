use std::fs::File;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::path::PathBuf;

pub fn play_sound_backup_ok() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .ancestors()
        .nth(1)  // Salta un solo livello per arrivare a Group-29
        .ok_or("Impossibile ottenere la directory del progetto")?;
    let file_path = project_dir.join("group29_backup/Suoni/successoBackup.wav");

    // Inizializza il flusso di output
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Crea un Sink per gestire il suono
    let sink = Sink::try_new(&stream_handle)?;

    // Carica il file audio
    let file = File::open(file_path)?;
    let source = Decoder::new_wav(file)?;

    // Aggiungi il suono al Sink
    sink.append(source);

    // Riproduci il suono
    sink.sleep_until_end(); // Aspetta fino a quando il suono finisce di riprodursi

    Ok(())
}

pub fn play_sound_backup_error() -> Result<(), Box<dyn std::error::Error>> {

    let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .ancestors()
        .nth(1)  // Salta un solo livello per arrivare a Group-29
        .ok_or("Impossibile ottenere la directory del progetto")?;
    let file_path = project_dir.join("group29_backup/Suoni/erroreBackup.wav");
    // Inizializza il flusso di output
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Crea un Sink per gestire il suono
    let sink = Sink::try_new(&stream_handle)?;

    // Carica il file audio
    let file = File::open(file_path)?;
    let source = Decoder::new_wav(file)?;

    // Aggiungi il suono al Sink
    sink.append(source);

    // Riproduci il suono
    sink.sleep_until_end(); // Aspetta fino a quando il suono finisce di riprodursi

    Ok(())
}
