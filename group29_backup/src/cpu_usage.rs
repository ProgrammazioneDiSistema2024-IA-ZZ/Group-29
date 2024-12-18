use sysinfo::{System, ProcessesToUpdate, Pid};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use std::thread;
use crate::dir_functions::get_project_directory;

pub fn log_cpu_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut system = System::new_all();

    // Ottieni il PID del processo corrente
    let pid = std::process::id();
    println!("PID del processo corrente: {}", pid);

    // Determina il percorso del file di log
    let proj_dir = get_project_directory()?;
    let file_path = proj_dir.join("cpu_usage_log.txt");
    println!("Percorso del file di log: {:?}", file_path);

    // Apre il file di log per scrittura nella directory specificata
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .expect("Impossibile aprire o creare il file di log");
    println!("File di log aperto correttamente.");

    // Ottieni il numero di core logici della CPU
    let num_cores = system.cpus().len();

    // Configura gli intervalli
    let logging_interval = Duration::from_secs(120); // Intervallo totale di logging
    let sampling_interval = Duration::from_secs(5);  // Intervallo di campionamento

    loop {
        let start_time = Instant::now();
        let mut total_cpu_usage = 0.0;
        let mut sample_count = 0;


        // Raccogli campioni durante l'intervallo di logging
        while start_time.elapsed() < logging_interval {
            let sample_start = Instant::now();

            // Aggiorna le informazioni sui processi
            system.refresh_processes(
                ProcessesToUpdate::Some(&[Pid::from(pid as usize)]),
                true,
            );

            // Ottieni l'uso della CPU del processo
            if let Some(process) = system.process(Pid::from(pid as usize)) {
                let cpu_usage = process.cpu_usage();
                

                // Accumula l'uso della CPU e incrementa il numero di campioni
                total_cpu_usage += cpu_usage;
                sample_count += 1;
            } else {
                println!("Impossibile trovare il processo con PID {}", pid);
            }

            // Aspetta l'intervallo di campionamento
            thread::sleep(sampling_interval);
        }

        // Calcola la media normalizzata
        if sample_count > 0 {
            let average_cpu_usage = total_cpu_usage / (sample_count as f32 * num_cores as f32);

            if let Err(e) = writeln!(
                file,
                "Media consumo CPU del processo {}: {:.6}% dopo {} campioni.",
                pid,
                average_cpu_usage,
                sample_count
            ) {
                println!("Errore durante la scrittura nel file: {}", e);
            } else {
                println!("Dati scritti correttamente nel file.");
            }

            // Forza il flush del file
            file.flush().expect("Errore durante il flush del file");
        } else {
            println!(
                "Nessun dato disponibile per calcolare la media del processo {}",
                pid
            );
            if let Err(e) = writeln!(
                file,
                "Nessun dato disponibile per calcolare la media del processo {}",
                pid
            ) {
                println!("Errore durante la scrittura nel file: {}", e);
            }
            file.flush().expect("Errore durante il flush del file");
        }

        println!("Fine dell'intervallo di logging.");
    }
}
