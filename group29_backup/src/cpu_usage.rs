use sysinfo::{System, ProcessesToUpdate, Pid};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use std::thread;

pub fn log_cpu_usage() {
    let mut system = System::new_all();

    // Ottieni il PID del processo corrente
    let pid = std::process::id();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("cpu_usage_log.txt")
        .expect("Impossibile aprire il file di log");

    loop {

        system.refresh_processes(
            ProcessesToUpdate::Some(&[Pid::from(pid as usize)]),
            true,
        );

        let pid = Pid::from(pid as usize);

        // Log dell'uso della CPU del processo corrente
        if let Some(process) = system.process(pid) {
            let process_cpu_usage = process.cpu_usage();
            writeln!(
                file,
                "Consumo di CPU del processo {}: {:.6}%",
                pid,
                process_cpu_usage
            )
                .expect("Impossibile scrivere nel file di log");
        } else {
            writeln!(file, "Impossibile trovare il processo con PID: {}", pid)
                .expect("Impossibile scrivere nel file di log");
        }

        // Attendi 120 secondi prima di ripetere il log
        thread::sleep(Duration::from_secs(120));
    }
}
