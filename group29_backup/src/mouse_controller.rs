use std::thread;
use crate::eventi::{check_movement};
use std::sync::{Arc,Mutex};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn; // Usa RunReturn per controllare l'event loop manualmente
enum State {
    Rectangle,
    MinusSign,
    Done
}

pub fn mouse_events() {
    println!("Sei in mouse events");

    let event_loop = EventLoop::new(); // Creiamo un event loop temporaneo
    let monitor = event_loop.primary_monitor().unwrap();
    let screen_size = monitor.size();
    let screen_width = screen_size.width;
    let screen_height = screen_size.height;

    let stop_flag = Arc::new(Mutex::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);

    println!("Screen size: {}x{}", screen_width, screen_height);

    //Gestire i thread dei due tracciamenti usando stati
    let stop_flag_for_loop = Arc::clone(&stop_flag);


    // Esegui `check_movement` in un altro thread
    thread::spawn(move || {
        check_movement(screen_width as f64, screen_height as f64, stop_flag_clone);
    });


    event_loop.run(move |event, _, control_flow| {
        // Controlla se il tracking ha settato il flag di stop
        let stop = stop_flag.lock().unwrap();
        if *stop {
            *control_flow = ControlFlow::Exit;
            println!("Exit event loop due to stop flag being set.");
        } else {
            *control_flow = ControlFlow::Wait;
        }

        // Aggiungi qui la gestione di altri eventi, se necessario
    });

}