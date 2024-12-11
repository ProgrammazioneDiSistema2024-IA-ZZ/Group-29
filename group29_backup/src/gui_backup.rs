use eframe::{egui, App}; // Assicurati di avere le importazioni corrette
use std::fs;
use std::path::PathBuf;
use native_dialog::FileDialog; // Per il dialogo di selezione della cartella
use egui::ViewportCommand;
use crate::dir_functions::get_project_directory;
pub struct ConfigApp {
    pub use_full_folder: bool,     // Checkbox per "Cartella completa del progetto"
    pub selected_extension: String, // Estensione selezionata
    pub input_path: String,        // Percorso di ingresso
    pub output_path: String,       // Percorso di uscita
}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
            use_full_folder: false,           // Valore predefinito
            selected_extension: "txt".to_string(), // Estensione predefinita
            input_path: String::new(),         // Percorso di ingresso vuoto
            output_path: String::new(),        // Percorso di uscita vuoto
        }
    }
}

impl App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Configurazione Applicativo");

            // Checkbox per selezionare l'opzione
            ui.checkbox(&mut self.use_full_folder, "Cartella completa");

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

            // Seleziona la cartella di ingresso
            ui.horizontal(|ui| {
                ui.label("Percorso di ingresso:");
                ui.text_edit_singleline(&mut self.input_path);

                if ui.button("Seleziona...").clicked() {
                    if let Some(path) = open_directory_dialog() {
                        self.input_path = path.to_string_lossy().to_string();
                        println!("input: {}", self.input_path)
                    }
                }
            });

            // Seleziona la cartella di uscita
            ui.horizontal(|ui| {
                ui.label("Percorso di uscita:");
                ui.text_edit_singleline(&mut self.output_path);

                if ui.button("Seleziona...").clicked() {
                    if let Some(path) = open_directory_dialog() {
                        self.output_path = path.to_string_lossy().to_string();
                        println!("output: {}", self.output_path)
                    }
                }
            });
            // Pulsante per applicare la configurazione e salvarla
            if ui.button("Salva configurazione").clicked() {
                if !self.input_path.is_empty() && !self.output_path.is_empty() {
                    match write_config(self.use_full_folder, &self.selected_extension, &self.input_path, &self.output_path) {
                        Ok(_) => {
                            println!("Configurazione aggiornata con successo.");
                        },
                        Err(e) => println!("Errore nell'aggiornamento della configurazione: {}", e),
                    }
                    println!(
                        "Modifiche applicate: Cartella completa = {}, Estensione = {}, Percorso Ingresso = {}, Percorso Uscita = {}",
                        self.use_full_folder, self.selected_extension, self.input_path, self.output_path
                    );
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                } else {
                    println!("Per favore, seleziona entrambi i percorsi di ingresso e uscita.");
                }
            }
        });
    }
}

fn write_config(use_full_folder: bool, extension: &str, input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_content = if use_full_folder {
        format!(
            r#"
backup_type = "FullFolder"
input_path = {:?}
output_path = {:?}
"#,
            input_path, output_path
        )
    } else {
        format!(
            r#"
backup_type = "FileType"
extension = {:?}
input_path = {:?}
output_path = {:?}
"#,
            extension, input_path, output_path
        )
    };
    let proj_dir = get_project_directory()?;
    let config_path = proj_dir.join("config.toml");
    //fs::write(config_path, config_content)?;
    println!("Config Path calcolato da write: {:?}", config_path);
    println!("Config Content: {}", config_content);
    fs::write(config_path, config_content).map_err(|e| {
        println!("Errore nella scrittura del file config.toml: {}", e);
        e
    })?;
    Ok(())
}


// Funzione per aprire il dialogo di selezione delle directory
fn open_directory_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("C:/")
        .show_open_single_dir()
        .ok()
        .flatten()
}
