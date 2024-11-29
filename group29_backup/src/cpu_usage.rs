use sysinfo::{System, ProcessesToUpdate, Pid};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use std::thread;

pub fn log_cpu_usage() {
    let mut system = System::new_all();

    // Ottieni il PID del processo corrente
    let pid = std::process::id();
    println!("PID del processo corrente: {}", pid);

    // Apre il file di log per scrittura
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("cpu_usage_log.txt")
        .expect("Impossibile aprire il file di log");
    println!("File di log aperto correttamente. Percorso: {:?}", std::env::current_dir());

    // Ottieni il numero di core logici della CPU
    let num_cores = system.cpus().len();
    println!("Numero di core logici: {}", num_cores);

    // Configura gli intervalli
    let logging_interval = Duration::from_secs(120); // Intervallo totale di logging
    let sampling_interval = Duration::from_secs(5);  // Intervallo di campionamento

    loop {
        let start_time = Instant::now();
        let mut total_cpu_usage = 0.0;
        let mut sample_count = 0;

        println!("Inizio raccolta dati per l'intervallo di logging.");

        // Raccogli campioni durante l'intervallo di logging
        while start_time.elapsed() < logging_interval {
            println!("Campionamento iniziato...");
            let sample_start = Instant::now();

            // Aggiorna le informazioni sui processi
            system.refresh_processes(
                ProcessesToUpdate::Some(&[Pid::from(pid as usize)]),
                true,
            );
            println!("Processi aggiornati.");

            // Ottieni l'uso della CPU del processo
            if let Some(process) = system.process(Pid::from(pid as usize)) {
                let cpu_usage = process.cpu_usage();
                println!(
                    "CPU usage campionato: {:.6}% (tempo campionamento: {:.6}s)",
                    cpu_usage,
                    sample_start.elapsed().as_secs_f32()
                );

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

            // Scrivi la media nel file di log
            println!(
                "Media consumo CPU calcolata: {:.6}% dopo {} campioni.",
                average_cpu_usage, sample_count
            );
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