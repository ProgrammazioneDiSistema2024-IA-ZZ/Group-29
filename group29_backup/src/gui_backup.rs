use eframe::{egui, App}; // Assicurati di avere le importazioni corrette
use std::fs;
use std::io::{self, Write};

pub struct ConfigApp {
    pub use_full_folder: bool, // Checkbox per "Cartella completa del progetto"
    pub selected_extension: String, // Estensione selezionata
}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
            use_full_folder: false, // Valore predefinito
            selected_extension: "txt".to_string(), // Estensione predefinita
        }
    }
}

// Funzione per scrivere nel file config.toml
fn write_config(use_full_folder: bool, extension: &str) -> Result<(), io::Error> {
    // Usa una Stringa per mantenere il contenuto
    let config_content = if use_full_folder {
        r#"
backup_type = { FullFolder = true }
"#.to_string() // Usa .to_string() per mantenere il valore
    } else {
        format!(r#"
backup_type = {{ FileType = "{}" }}
"#, extension) // Usa format! per generare il contenuto
    };

    // Modificato il percorso per scrivere nel file config.toml
    fs::write("../config.toml", config_content)?; // Usa ../ per salire di un livello
    Ok(())
}

impl App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Configurazione Applicativo");

            // Checkbox per selezionare l'opzione
            ui.checkbox(&mut self.use_full_folder, "Cartella completa del progetto");

            // Mostra il ComboBox se l'utente non ha selezionato "Cartella completa del progetto"
            if !self.use_full_folder {
                ui.label("Seleziona l'estensione dei file:");
                egui::ComboBox::from_label("Estensioni")
                    .selected_text(&self.selected_extension)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_extension, "txt".to_string(), "Testo (.txt)");
                        ui.selectable_value(&mut self.selected_extension, "toml".to_string(), "TOML (.toml)");
                    });
            }

            // Pulsante per applicare le modifiche
            if ui.button("Salva configurazione").clicked() {
                match write_config(self.use_full_folder, &self.selected_extension) {
                    Ok(_) => {
                        println!("Configurazione aggiornata con successo.");
                        frame.close(); // Chiudi la GUI
                    },
                    Err(e) => println!("Errore nell'aggiornamento della configurazione: {}", e),
                }
                println!(
                    "Modifiche applicate: Cartella completa = {}, Estensione = {}",
                    self.use_full_folder, self.selected_extension
                );
            }
        });
    }
}

