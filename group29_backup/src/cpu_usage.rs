use sysinfo::{System, SystemExt, ProcessExt, Pid, CpuExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

pub fn log_cpu_usage() {
    let mut system = System::new_all();

    // Ottieni il PID del processo corrente
    let pid = Pid::from(std::process::id() as usize);

    loop {
        // Log dell'uso della CPU totale
        let cpu_usage = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("cpu_usage.log")
            .expect("Impossibile aprire il file di log");


        // Log dell'uso della CPU del processo corrente
        if let Some(process) = system.process(pid) {
            let process_cpu_usage = process.cpu_usage();

            writeln!(file, "Consumo di CPU del processo: {:.2}%", process_cpu_usage)
                .expect("Impossibile scrivere nel file di log");
        } else {
            writeln!(file, "Impossibile trovare il processo con PID: {}", pid)
                .expect("Impossibile scrivere nel file di log");
        }

        // Attendi 120 secondi prima di ripetere il log
        thread::sleep(Duration::from_secs(120));
    }
    }

