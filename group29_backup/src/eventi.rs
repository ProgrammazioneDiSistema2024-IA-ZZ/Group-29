use rdev::{listen, Event, EventType};

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
