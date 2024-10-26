use std::thread;
use crate::eventi::{track_minus_sign, check_movement};
use std::sync::{mpsc,Arc,Mutex};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn mouse_events() {
    println!("Sei in mouse events");

    let event_loop = winit::event_loop::EventLoop::new(); // Creiamo un event loop temporaneo
    let monitor = event_loop.primary_monitor().unwrap();
    let screen_size = monitor.size();
    let screen_width = screen_size.width;
    let screen_height = screen_size.height;

    println!("Screen size: {}x{}", screen_width, screen_height);

    //Gestire i thread dei due tracciamenti usando stati
    let should_continue = Arc::new(Mutex::new(true));

    // This handles the event listening loop
    let should_continue_clone = Arc::clone(&should_continue);

    thread::spawn( move || {
        check_movement(screen_width as f64,screen_height as f64, should_continue_clone);
    });

    event_loop.run(move|_event,_,control_flow|{
        *control_flow = winit::event_loop::ControlFlow::Wait;
    });
}