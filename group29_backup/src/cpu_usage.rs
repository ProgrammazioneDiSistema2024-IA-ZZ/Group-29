use sysinfo::{System, SystemExt, ProcessExt, Pid};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

pub fn log_cpu_usage() {
    let mut system = System::new_all();
    /*
    let pid = Pid::from(std::process::id() as usize);
    //COMMIT
    loop {
        system.refresh_process(pid);

        // Ottieni il processo corrente usando il PID
        if let Some(process) = system.process(pid) {
            let cpu_usage = process.cpu_usage(); // Uso della CPU del processo corrente

            // Apri o crea il file di log
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("process_cpu_usage.log")
                .expect("Impossibile aprire il file di log");

            // Scrivi l'uso della CPU nel file di log
            writeln!(file, "Consumo di CPU del processo: {:.2}%", cpu_usage)
                .expect("Impossibile scrivere nel file di log");
        } else {
            eprintln!("Impossibile trovare il processo con PID: {}", pid);
        }

        */
        let cpu_usage = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("cpu_usage.log")
            .expect("Impossibile aprire il file di log");

        writeln!(file, "Consumo di CPU: {:.2}%", cpu_usage)
            .expect("Impossibile scrivere nel file di log");



        // Attendi 10 secondi prima di ripetere il log
        thread::sleep(Duration::from_secs(120));
    }
}
