use rdev::{listen, Event};

// Funzione per gestire gli eventi del mouse
pub fn handle_mouse_events() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

// Funzione di callback per gestire diversi tipi di eventi
fn callback(event: Event) {
    match event {
        Event::MouseMove { x, y } => {
            println!("Mouse moved to: ({}, {})", x, y);
        }
        Event::ButtonPress(button) => {
            println!("Mouse button pressed: {:?}", button);
        }
        Event::ButtonRelease(button) => {
            println!("Mouse button released: {:?}", button);
        }
        _ => (),
    }
}