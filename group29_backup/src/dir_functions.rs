use std::env;
use std::path::PathBuf;

pub fn get_project_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?; // Ottieni il percorso dell'eseguibile
    let mut project_dir = exe_path
        .parent() // Ottieni la directory dell'eseguibile
        .ok_or("Impossibile ottenere la directory del progetto.")?
        .to_path_buf();

    // Controlla se il percorso termina con "group29_backup"
    while !project_dir.ends_with("group29_backup") {
        project_dir = project_dir
            .parent() // Risali alla directory superiore
            .ok_or("Non è stato possibile trovare la directory group29_backup.")?
            .to_path_buf();
    }

    println!("Directory del progetto calcolata: {:?}", project_dir);

    Ok(project_dir)
}

pub fn get_project_directory_sound() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?; // Ottieni il percorso dell'eseguibile
    let mut project_dir = exe_path
        .parent() // Ottieni la directory dell'eseguibile
        .ok_or("Impossibile ottenere la directory del progetto.")?
        .to_path_buf();

    // Controlla se il percorso termina con "group29_backup"
    while !project_dir.ends_with("group29_backup") {
        project_dir = project_dir
            .parent() // Risali alla directory superiore
            .ok_or("Non è stato possibile trovare la directory group29_backup.")?
            .to_path_buf();
    }

    let result = project_dir.to_path_buf().join("Suoni");
    println!("Path suoni: {:?}", result);
    Ok(result)
}