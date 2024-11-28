use std::env;
use std::path::PathBuf;

pub fn get_project_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .ancestors()
        .nth(1)
        .ok_or("Impossibile ottenere la directory del progetto")?;
    println!("Project Directory: {:?}", project_dir);
    Ok(project_dir.to_path_buf())
}

pub fn get_project_directory_sound() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .ancestors()
        .nth(2)
        .ok_or("Impossibile ottenere la directory del progetto")?;
    let result = project_dir.to_path_buf().join("Suoni");
    println!("Project Directory suoni: {:?}", project_dir);
    Ok(result)
}