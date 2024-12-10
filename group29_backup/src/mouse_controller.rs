use std::path::PathBuf;
use std::thread;
use crate::eventi::{ check_movement};
use std::sync::{mpsc,Arc,Mutex};
use winit::event_loop::EventLoop;
use std::sync::atomic::{Ordering,AtomicBool};
use crate::backup;
use std::time::{Instant};
use rdev::{Event, listen};
use crate::suoni::play_sound_sign;


//const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(50); // Intervallo di debounce per ignorare eventi troppo vicini

pub fn mouse_events(extension: Option<String>, backup_type: &String, input_path: &String, output_path: &String ) {
    let (screen_width, screen_height) = {
        let event_loop = EventLoop::new();
        let monitor = event_loop.primary_monitor().unwrap();
        let screen_size = monitor.size();
        (screen_size.width, screen_size.height)
    };
    let done_flag = Arc::new(AtomicBool::new(false));
    let (done_sender,done_receiver) = mpsc::channel();

    //Avvia il controllo del movimento
    let done_flag_clone = Arc::clone(&done_flag);
    let last_event_time = Arc::new(Mutex::new(Instant::now())); // Per tenere traccia dell'ultimo evento significativo

    thread::spawn(move || {
        listen(move |event: Event| {
            check_movement(screen_width as f64, screen_height as f64, Arc::clone(&done_flag_clone), done_sender.clone());
        }).expect("Errore nell'ascolto degli eventi di rdev in mouse_events");
    });


    if(done_receiver).recv().is_ok(){
        match play_sound_sign() {
            Ok(_) => println!("Suono riprodotto con successo"),
            Err(e) => eprintln!("Errore durante la riproduzione del suono: {}", e),
        }
        done_flag.store(true,Ordering::Relaxed);
        backup::perform_backup(backup_type, extension.as_deref(), &PathBuf::from(input_path), &PathBuf::from(output_path)).expect("Errore durante il backup");
    }
}

