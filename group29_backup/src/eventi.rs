use rdev::{listen, Event, EventType};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::monitor::MonitorHandle;
use winit::window::WindowBuilder; // Ensure this import is correct

// Funzione per gestire gli eventi del mouse
pub fn handle_mouse_events() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

// Funzione di callback per gestire diversi tipi di eventi
fn callback(event: Event) {
    match event.event_type {
        EventType::MouseMove { x, y } => {
            println!("Mouse moved to: ({}, {})", x, y);
        }
        EventType::ButtonPress(button) => {
            println!("Mouse button pressed: {:?}", button);
        }
        EventType::ButtonRelease(button) => {
            println!("Mouse button released: {:?}", button);
        }
        _ => (),
    }
}
// Funzione per ottenere la dimensione dello schermo e monitor principale
pub fn get_screen_dimensions() {
    // Crea un nuovo EventLoop
    let event_loop = EventLoop::new();

    // Crea una finestra invisibile
    let window = WindowBuilder::new()
        .with_visible(false) // La finestra è nascosta perché non ci serve
        .build(&event_loop)
        .expect("Impossibile creare la finestra");

    // Ottiene il monitor principale
    let monitor = window.primary_monitor();

    // Recupera la risoluzione dello schermo
    let (screen_width, screen_height) = match monitor {
        Some(monitor) => {
            let size = monitor.size();
            (size.width as f64, size.height as f64)
        }
        None => {
            println!("Impossibile ottenere la risoluzione dello schermo!");
            return;
        }
    };

    // Stampa le dimensioni dello schermo
    println!("Risoluzione dello schermo: {}x{}", screen_width, screen_height);
}

// Funzione principale del programma

