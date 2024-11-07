use std::thread;
use crate::eventi::{track_minus_sign, check_movement};
use std::sync::{mpsc,Arc,Mutex};
use winit::event_loop::{ EventLoop};
use std::sync::atomic::{Ordering,AtomicBool};

pub fn mouse_events() {
    println!("Sei in mouse events");

    //Per prendere dimensioni schermo
    let event_loop = winit::event_loop::EventLoop::new(); // Creiamo un event loop temporaneo
    let monitor = event_loop.primary_monitor().unwrap();
    let screen_size = monitor.size();
    let screen_width = screen_size.width;
    let screen_height = screen_size.height;

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
    }


}