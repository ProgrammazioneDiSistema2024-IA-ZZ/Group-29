use std::env;
use std::path::PathBuf;

pub fn get_project_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let mut project_dir = exe_path
        .parent()
        .ok_or("Impossibile ottenere la directory del progetto.")?
        .to_path_buf();

    //controllo per MACOS:
    if project_dir.ends_with("MacOS") {
        project_dir = project_dir
            .parent()
            .and_then(|p| p.parent())
            .ok_or("Non è stato possibile risalire alla directory del progetto dal bundle macOS.")?
            .to_path_buf();
    }

    // Controlla se il percorso termina con "group29_backup"
    while !project_dir.ends_with("group29_backup") {
        project_dir = project_dir
            .parent()
            .ok_or("Non è stato possibile trovare la directory group29_backup.")?
            .to_path_buf();
    }

    println!("Directory del progetto calcolata: {:?}", project_dir);

    Ok(project_dir)
}

pub fn get_project_directory_sound() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let mut project_dir = exe_path
        .parent()
        .ok_or("Impossibile ottenere la directory del progetto.")?
        .to_path_buf();


    while !project_dir.ends_with("group29_backup") {
        project_dir = project_dir
            .parent()
            .ok_or("Non è stato possibile trovare la directory group29_backup.")?
            .to_path_buf();
    }

    let result = project_dir.to_path_buf().join("Suoni");
    println!("Path suoni: {:?}", result);
    Ok(result)
}