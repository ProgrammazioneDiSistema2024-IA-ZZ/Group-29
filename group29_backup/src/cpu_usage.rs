use sysinfo::{System, SystemExt, CpuExt};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

pub fn log_cpu_usage() {
    let mut system = System::new_all();

    loop {
        // Aggiorna le informazioni del sistema, inclusa la CPU
        system.refresh_cpu();

        // Ottieni il consumo globale della CPU (media di tutti i core)
        let cpu_usage = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / system.cpus().len() as f32;

        // Scrivi il consumo di CPU su un file di log
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("cpu_usage.log")
            .expect("Impossibile aprire il file di log");

        writeln!(file, "Consumo di CPU: {:.2}%", cpu_usage).expect("Impossibile scrivere nel file di log");

        // Attendi 2 minuti prima di ripetere il log
        thread::sleep(Duration::from_secs(10));
    }
}
