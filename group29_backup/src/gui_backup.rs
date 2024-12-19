use eframe::{egui, App}; // Assicurati di avere le importazioni corrette
use std::fs;
use std::path::PathBuf;
use native_dialog::FileDialog; // Per il dialogo di selezione della cartella
use std::env;

pub struct ConfigApp {
    pub use_full_folder: bool,     // Checkbox per "Cartella completa del progetto"
    pub selected_extension: String, // Estensione selezionata
    pub input_path: String,        // Percorso di ingresso
    pub output_path: String,       // Percorso di uscita
}

impl Default for ConfigApp {
    fn default() -> Self {
        Self {
            use_full_folder: false,
            selected_extension: "txt".to_string(),
            input_path: String::new(),
            output_path: String::new(),
        }
    }
}

impl App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Configurazione Applicativo");

            ui.checkbox(&mut self.use_full_folder, "Cartella completa");

            if !self.use_full_folder {
                ui.label("Seleziona l'estensione dei file:");
                egui::ComboBox::from_label("Estensioni")
                    .selected_text(&self.selected_extension)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_extension, "txt".to_string(), "Testo (.txt)");
                        ui.selectable_value(&mut self.selected_extension, "toml".to_string(), "TOML (.toml)");
                    });
            }

            ui.horizontal(|ui| {
                ui.label("Percorso di ingresso:");
                ui.text_edit_singleline(&mut self.input_path);

                if ui.button("Seleziona...").clicked() {
                    if let Some(path) = open_directory_dialog() {
                        self.input_path = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Percorso di uscita:");
                ui.text_edit_singleline(&mut self.output_path);

                if ui.button("Seleziona...").clicked() {
                    if let Some(path) = open_directory_dialog() {
                        self.output_path = path.to_string_lossy().to_string();
                    }
                }
            });

            if ui.button("Salva configurazione").clicked() {
                if !self.input_path.is_empty() && !self.output_path.is_empty() {
                    match write_config(
                        self.use_full_folder,
                        &self.selected_extension,
                        &self.input_path,
                        &self.output_path,
                    ) {
                        Ok(_) => {
                            println!("Configurazione aggiornata con successo.");
                            frame.close();
                        }
                        Err(e) => println!("Errore nell'aggiornamento della configurazione: {}", e),
                    }
                } else {
                    println!("Seleziona entrambi i percorsi di ingresso e uscita.");
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

    fs::write(config_path, config_content)?;
    Ok(())
}

fn open_directory_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .set_location(&env::current_dir().unwrap_or_else(|_| PathBuf::from("/")))
        .show_open_single_dir()
        .ok()
        .flatten()
}

pub fn get_project_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let mut project_dir = exe_path
        .parent()
        .ok_or("Impossibile ottenere la directory del progetto.")?
        .to_path_buf();

    if project_dir.ends_with("MacOS") {
        project_dir = project_dir
            .parent()
            .and_then(|p| p.parent())
            .ok_or("Directory non trovata per macOS.")?
            .to_path_buf();
    }

    while !project_dir.ends_with("group29_backup") {
        project_dir = project_dir
            .parent()
            .ok_or("Directory group29_backup non trovata.")?
            .to_path_buf();
    }

    Ok(project_dir)
}

pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Backup Configurator",
        options,
        Box::new(|_cc| Box::new(ConfigApp::default())),
    )
    .expect("Errore nell'avvio della GUI");
}

fn main() {
    run_gui();
}