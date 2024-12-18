use std::fs::File;
use rodio::{Decoder, OutputStream, Sink};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;
use crate::dir_functions::get_project_directory;
pub fn play_sound_backup_ok() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = get_project_directory()?;
    println!("Project Directory suoni: {:?}", project_dir);
    let file_path = project_dir.join("successoBackup.wav");
    println!("Path suono successoooooooo: {:?}", file_path);
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
    let project_dir = get_project_directory()?;
    println!("Project Directory suoni: {:?}", project_dir);
    let file_path = project_dir.join("erroreBackup.wav");
    println!("Path suono successoooooooo: {:?}", file_path);
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
pub fn play_sound_sign() -> Result<(), Box<dyn std::error::Error>> {
    // Configura la frequenza del suono e la frequenza di campionamento
    let sample_rate = 44100.0;
    let frequency = 550.0; // Frequenza di La (A4)

    // Ottieni il dispositivo di output audio predefinito
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Nessun dispositivo di output disponibile");
    let config = device.default_output_config().expect("Nessuna configurazione di output disponibile");

    // Funzione per generare l'onda sinusoidale
    let mut sample_clock = 0f32;
    let sample_delta = frequency * 2.0 * PI / sample_rate;

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = (sample_clock * sample_delta).sin();
                sample_clock = (sample_clock + 1.0) % sample_rate;
            }
        },
        |err| eprintln!("Errore nel flusso audio: {}", err),
    ).expect("Errore nella creazione del flusso audio");

    // Avvia il flusso
    stream.play().expect("Errore nell'avvio del flusso audio");

    // Mantiene il programma in esecuzione per ascoltare il suono
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}