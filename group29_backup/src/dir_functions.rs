use std::env;
use std::path::PathBuf;

pub fn get_project_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    /*let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .ancestors()
        .nth(1)
        .ok_or("Impossibile ottenere la directory del progetto")?;*/
    let exe_path = env::current_exe()?;
    // Risale alla directory del progetto partendo dall'eseguibile
    let project_dir = exe_path
        .parent() // Ottieni la directory dell'eseguibile
        .ok_or("Impossibile ottenere la directory del progetto.")?
        .to_path_buf();

    println!("Directory del progetto calcolata: {:?}", project_dir);

    Ok(project_dir)
    //Ok(current_dir.to_path_buf())
}

pub fn get_project_directory_sound() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let result = current_dir.to_path_buf().join("Suoni");
    println!("Project Directory suoni: {:?}", result);
    Ok(result)
}