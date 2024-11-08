use std::path::PathBuf;
use std::thread;
use crate::eventi::{track_minus_sign, check_movement};
use std::sync::{mpsc,Arc,Mutex};
use winit::event_loop::{ EventLoop};
use std::sync::atomic::{Ordering,AtomicBool};
use std::time::Duration;
use crate::backup;


pub fn mouse_events(extension: Option<String>,backup_type: &String,input_path: &String ,  output_path: &String ) {
    println!("Sei in mouse events");

    let (screen_width, screen_height) = {
        let event_loop = EventLoop::new();
        let monitor = event_loop.primary_monitor().unwrap();
        let screen_size = monitor.size();
        (screen_size.width, screen_size.height)
    };

    println!("Screen size: {}x{}", screen_width, screen_height);

    let done_flag = Arc::new(AtomicBool::new(false));
    let (done_sender,done_receiver) = mpsc::channel();

    //Avvia il controllo del movimento
    let done_flag_clone = Arc::clone(&done_flag);
    thread::spawn(move || {
        check_movement(screen_width as f64,screen_height as f64,done_flag_clone,done_sender);
    });

    if(done_receiver).recv().is_ok(){
        println!("Movimento e segno meno rilevati. Esecuzione commpletata");
        done_flag.store(true,Ordering::Relaxed);
        backup::perform_backup(backup_type, extension.as_deref(), &PathBuf::from(input_path), &PathBuf::from(output_path));

    }

}