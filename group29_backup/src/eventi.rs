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
/* Versione per registrare il rettangolo premento il mouse
    use rdev::{listen, Event, EventType};
    use winit::platform::windows::EventLoopExtWindows;
    use winit::event_loop::EventLoop;
    use winit::monitor::MonitorHandle;

    // Funzione per tracciare i punti del mouse quando è premuto
    pub fn handle_mouse_events() {
        // Ottieni la dimensione dello schermo dinamicamente usando winit
        let event_loop = EventLoop::new();
        let monitor = get_primary_monitor(&event_loop);

        let (screen_width, screen_height) = match monitor {
            Some(monitor) => {
                let size = monitor.size();
                (size.width as f64, size.height as f64)
            },
            None => {
                println!("Impossibile ottenere la risoluzione dello schermo!");
                return;
            }
        };

        println!("Risoluzione dello schermo: {}x{}", screen_width, screen_height);

        let mut points = Vec::new(); // Lista per memorizzare i punti del mouse
        let mut mouse_pressed = false; // Flag per sapere se il mouse è premuto

        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    if mouse_pressed {
                        // Registra i punti solo se il mouse è premuto
                        points.push((x, y));
                        println!("Mouse moved to: ({}, {})", x, y);
                    }
                }
                EventType::ButtonPress(_) => {
                    mouse_pressed = true; // Inizia a registrare quando il mouse è premuto
                    points.clear(); // Pulisci i punti per una nuova traccia
                    println!("Mouse button pressed");
                }
                EventType::ButtonRelease(_) => {
                    mouse_pressed = false; // Ferma la registrazione quando il mouse viene rilasciato
                    if is_rectangle_along_screen_borders(&points, screen_width, screen_height) {
                        println!("Rettangolo lungo i bordi del monitor riconosciuto!");
                        // Qui puoi avviare il backup
                    } else {
                        println!("Nessun rettangolo riconosciuto lungo i bordi del monitor.");
                    }
                    println!("Mouse button released");
                }
                _ => (),
            }
        }) {
            println!("Error: {:?}", error);
        }
    }

    // Funzione per ottenere il monitor principale e le dimensioni
    fn get_primary_monitor(event_loop: &EventLoop<()>) -> Option<MonitorHandle> {
        let monitors = event_loop.available_monitors();
        for monitor in monitors {
            return Some(monitor); // Prendi il primo monitor disponibile
        }
        None
    }

    // Funzione per riconoscere se i punti formano un rettangolo lungo i bordi dello schermo
    fn is_rectangle_along_screen_borders(points: &[(f64, f64)], screen_width: f64, screen_height: f64) -> bool {
        if points.len() < 4 {
            return false; // Troppi pochi punti per definire un rettangolo
        }

        // Definiamo una tolleranza per considerare i punti vicini ai bordi del monitor
        let tolerance = 20.0;

        let mut has_top_edge = false;
        let mut has_right_edge = false;
        let mut has_bottom_edge = false;
        let mut has_left_edge = false;

        for &(x, y) in points {
            // Verifica se il mouse segue il bordo superiore
            if y.abs() < tolerance {
                has_top_edge = true;
            }
            // Verifica se il mouse segue il bordo destro
            if (screen_width - x).abs() < tolerance {
                has_right_edge = true;
            }
            // Verifica se il mouse segue il bordo inferiore
            if (screen_height - y).abs() < tolerance {
                has_bottom_edge = true;
            }
            // Verifica se il mouse segue il bordo sinistro
            if x.abs() < tolerance {
                has_left_edge = true;
            }
        }

        // Verifica se sono stati toccati tutti e 4 i bordi del monitor
        has_top_edge && has_right_edge && has_bottom_edge && has_left_edge
    }

 */