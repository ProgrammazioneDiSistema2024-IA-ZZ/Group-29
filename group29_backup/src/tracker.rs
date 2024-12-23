use std::time::Duration;
use rdev::{Event, EventType};

// Costante per il debounce degli eventi
pub const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(100);

/// Rappresenta i possibili bordi dello schermo
#[derive(Debug)]
enum Border {
    None,
    Top,
    Right,
    Bottom,
    Left,
}

/// Rappresenta i possibili angoli dello schermo
#[derive(PartialEq, Debug)]
enum Corner {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Direzione del movimento
#[derive(PartialEq, Debug)]
enum Direction {
    Unknown,
    Clockwise,
    CounterClockwise,
}

/// Gestisce lo stato di tracciamento del rettangolo
#[derive(Debug)]
pub struct RectangleTracker {
    is_rectangle: bool,
    prev_x: f64,
    prev_y: f64,
    current_border: Border,
    direction: Direction,
    corner_reached: bool,
    initial_x: f64,
    initial_y: f64,
    initial_corner: Corner,
    flag_fine: bool,
    count_corners: i32,
    last_corner:Corner,
}

impl RectangleTracker {
    pub fn new() -> Self {
        Self {
            is_rectangle: false,
            prev_x: 0.0,
            prev_y: 0.0,
            current_border: Border::None,
            direction: Direction::Unknown,
            corner_reached: false,
            initial_x: 0.0,
            initial_y: 0.0,
            initial_corner: Corner::None,
            flag_fine: false,
            count_corners: 0,
            last_corner: Corner::None
        }
    }
}

/// Gestisce lo stato di tracciamento del segno meno
#[derive(Debug)]
pub struct MinusSignTracker {
    is_tracking: bool,
    initial_x: f64,
    initial_y: f64,
    prev_x: f64,
    is_minus_sign: bool,
}

impl MinusSignTracker {
    pub fn new() -> Self {
        Self {
            is_tracking: false,
            initial_x: 0.0,
            initial_y: 0.0,
            prev_x: 0.0,
            is_minus_sign: false,
        }
    }
}

///RECTANGLE TRACKER


/// Verifica se il mouse è in un angolo dello schermo
fn detect_corner(x: f64, y: f64, screen_width: f64, screen_height: f64) -> Corner {
    let tolerance = 50.0;
    println!("Verifica angolo: x = {}, y = {}", x, y);
    if x.abs() < tolerance && y.abs() < tolerance {
        println!("Angolo rilevato: TopLeft");
        Corner::TopLeft
    } else if (x - screen_width).abs() < tolerance && y.abs() < tolerance {
        Corner::TopRight
    } else if x.abs() < tolerance && (y - screen_height).abs() < tolerance {
        Corner::BottomLeft
    } else if (x - screen_width).abs() < tolerance && (y - screen_height).abs() < tolerance {
        Corner::BottomRight
    } else {
        Corner::None
    }
}

/// Traccia il movimento del mouse per verificare un rettangolo
pub fn track_rectangle(tracker: &mut RectangleTracker, screen_width: f64, screen_height: f64, event: Event) -> bool {
    let tolerance = 50.0; // Tolleranza di 5 pixel
    if let EventType::MouseMove { x, y } = event.event_type {
        println!("Coordinate  x = {}, y = {}", x, y);

        let corner = detect_corner(x, y, screen_width, screen_height);
       
        if corner != Corner::None && !tracker.corner_reached {
            //Prima iterazione -> Possibilità di avere una generazione di un rettangolo
            tracker.is_rectangle = true;
            tracker.initial_x = x;
            tracker.initial_y = y;
            tracker.prev_x = x;
            tracker.prev_y = y;
            tracker.corner_reached = true;
            tracker.initial_corner = corner;
            tracker.count_corners +=1;
            println!("Mouse rilevato nell'angolo: {:?}", tracker.initial_corner);
        } else if tracker.is_rectangle && tracker.direction == Direction::Unknown && corner == Corner::None {
            // Determina la direzione (ClockWise/Counterclockwise) e il bordo corrente basandosi sull'angolo iniziale
            match tracker.initial_corner {
                Corner::TopLeft => {
                    println!("X:{}, Y:{}", x, y);
                    if y.abs() < tolerance && x.abs() >= tolerance {
                        tracker.direction = Direction::Clockwise;
                        tracker.current_border = Border::Top;
                    } else if x.abs() < tolerance && y.abs() >= tolerance {
                        tracker.direction = Direction::CounterClockwise;
                        tracker.current_border = Border::Left;
                    }
                }
                Corner::TopRight => {
                    if (x - screen_width).abs() < tolerance && y.abs() >= tolerance {
                        tracker.direction = Direction::Clockwise;
                        tracker.current_border = Border::Right;
                    } else if y.abs() < tolerance && (x - screen_width).abs() >= tolerance {
                        tracker.direction = Direction::CounterClockwise;
                        tracker.current_border = Border::Top;
                    }
                }
                Corner::BottomRight => {
                    if (y - screen_height).abs() < tolerance && (x - screen_width).abs() >= tolerance {
                        tracker.direction = Direction::Clockwise;
                        tracker.current_border = Border::Bottom;
                    } else if (x - screen_width).abs() < tolerance && (y - screen_height).abs() >= tolerance {
                        tracker.direction = Direction::CounterClockwise;
                        tracker.current_border = Border::Right;
                    }
                }
                Corner::BottomLeft => {
                    if x.abs() < tolerance && (y - screen_height).abs() >= tolerance {
                        tracker.direction = Direction::Clockwise;
                        tracker.current_border = Border::Left;
                    } else if (y - screen_height).abs() < tolerance && x.abs() >= tolerance {
                        tracker.direction = Direction::CounterClockwise;
                        tracker.current_border = Border::Bottom;
                    }
                }
                Corner::None => println!("Angolo iniziale non valido"),
            }
            tracker.prev_x = x;
            tracker.prev_y = y;
        } else if tracker.is_rectangle && tracker.direction != Direction::Unknown && corner == Corner::None {
            // Traccia il movimento lungo i bordi in base alla direzione e al bordo corrente
            tracker.flag_fine = true; // Solo completando il rettangolo può chiudere
            match tracker.current_border {
                Border::Top => handle_top_border(tracker, x, y, tolerance, screen_width),
                Border::Right => handle_right_border(tracker, x, y, tolerance, screen_width, screen_height),
                Border::Bottom => handle_bottom_border(tracker, x, y, tolerance, screen_height),
                Border::Left => handle_left_border(tracker, x, y, tolerance, screen_height),
                Border::None => println!("Non su un bordo valido"),
            }
        } else {
            // Gestisce il passaggio di bordo nell'intorno di un angolo -> Corner!=None
            println!(
                "Mouse in {:?} neighbourhood ({}, {}), is_rectangle: {}, direction: {:?}",
                corner, x, y, tracker.is_rectangle, tracker.direction
            );
            if tracker.initial_corner == corner && tracker.flag_fine && tracker.count_corners > 2 {
                println!("Rettangolo completato!");
                tracker.flag_fine = false;
                return true;
            } else {
                switch_border(tracker, corner);
            }
        }
    }
    return false;
}


fn handle_top_border(tracker: &mut RectangleTracker, x: f64, y: f64, tolerance: f64, screen_width: f64) {
    match tracker.direction {
        Direction::Clockwise => {
            println!(
                "Movimento bordo superiore (Clockwise): prev_x = {}, x = {}, y = {}, tolerance = {}",
                tracker.prev_x, x, y, tolerance
            );
            if y.abs() < tolerance && tracker.prev_x <= x && x < screen_width {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo superiore fallito");
            }
        }
        Direction::CounterClockwise => {
            if y.abs() < tolerance && tracker.prev_x >= x && x >= 0.0 {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo superiore fallito");
            }
        }
        Direction::Unknown => println!("Problema con la direzione!"),
    }
}

fn handle_right_border(tracker: &mut RectangleTracker, x: f64, y: f64, tolerance: f64, screen_width: f64, screen_height: f64) {
    match tracker.direction {
        Direction::Clockwise => {
            if (x - screen_width).abs() < tolerance && tracker.prev_y <= y && y < screen_height {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo destro fallito");
            }
        }
        Direction::CounterClockwise => {
            if (x - screen_width).abs() < tolerance && tracker.prev_y >= y && y > 0.0 {
                tracker.prev_y = y;
            } else {
                reset_tracker(tracker, "Controllo bordo destro fallito");
            }
        }
        Direction::Unknown => println!("Problema con la direzione!"),
    }
}

fn handle_bottom_border(tracker: &mut RectangleTracker, x: f64, y: f64, tolerance: f64, screen_height: f64) {
    match tracker.direction {
        Direction::Clockwise => {
            if (y - screen_height).abs() < tolerance && tracker.prev_x >= x && x > 0.0 {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo inferiore fallito");
            }
        }
        Direction::CounterClockwise => {
            if (y - screen_height).abs() < tolerance && tracker.prev_x <= x && x < screen_height {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo inferiore fallito");
            }
        }
        Direction::Unknown => println!("Problema con la direzione!"),
    }
}

fn handle_left_border(tracker: &mut RectangleTracker, x: f64, y: f64, tolerance: f64, screen_height: f64) {
    match tracker.direction {
        Direction::Clockwise => {
            if x.abs() < tolerance && tracker.prev_y >= y && y > 0.0 {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo sinistro fallito");
            }
        }
        Direction::CounterClockwise => {
            if x.abs() < tolerance && tracker.prev_y <= y && y < screen_height {
                tracker.prev_y = y;
                tracker.prev_x = x;
            } else {
                reset_tracker(tracker, "Controllo bordo sinistro fallito");
            }
        }
        Direction::Unknown => println!("Problema con la direzione!"),
    }
}

fn reset_tracker(tracker: &mut RectangleTracker, message: &str) {
    tracker.flag_fine = false;
    tracker.is_rectangle = false;
    tracker.direction = Direction::Unknown;
    tracker.corner_reached = false;
    tracker.count_corners =0;
    println!("{}", message);
}

fn switch_border(tracker: &mut RectangleTracker, corner: Corner) {
    println!(
        "Cambio di bordo: corner = {:?}, last_corner = {:?}, direction = {:?}, count_corners = {}",
        corner, tracker.last_corner, tracker.direction, tracker.count_corners
    );
    if tracker.last_corner !=  corner && corner != Corner::None {
        match tracker.direction {
            Direction::Clockwise => {
                tracker.current_border = match corner {
                    Corner::TopLeft => Border::Top,
                    Corner::TopRight => Border::Right,
                    Corner::BottomLeft => Border::Left,
                    Corner::BottomRight => Border::Bottom,
                    Corner::None => Border::None,
                };
                tracker.count_corners += 1; // Incrementa il contatore
            }
            Direction::CounterClockwise => {
                tracker.current_border = match corner {
                    Corner::TopLeft => Border::Left,
                    Corner::TopRight => Border::Top,
                    Corner::BottomLeft => Border::Bottom,
                    Corner::BottomRight => Border::Right,
                    Corner::None => Border::None,
                };

                tracker.count_corners += 1; // Incrementa il contatore
            }
            Direction::Unknown => println!("Direzione sconosciuta"),
        }
        
        tracker.last_corner = corner; //Aggiorna l'ultimo angolo
    }
}



//MINUS SIGN TRACKER

/// Rileva un segno meno tramite il movimento del mouse
pub fn detect_minus_sign(tracker: &mut MinusSignTracker, screen_width: f64, screen_height: f64, event: Event) -> bool {
    let tolerance = 50.0;
    let min_length = screen_width * 0.2;

    if let EventType::MouseMove { x, y } = event.event_type {
        if !tracker.is_tracking {
            tracker.is_tracking = true;
            tracker.initial_x = x;
            tracker.initial_y = y;
            tracker.prev_x = x;
            println!("Inizio tracciamento segno meno: ({}, {})", x, y);
        } else if (tracker.initial_y - y).abs() < tolerance {
            tracker.prev_x = x;
            if (tracker.prev_x - tracker.initial_x) >= min_length {
                println!("Segno meno rilevato!");
                tracker.is_minus_sign = true;
                return true;
            }
        } else {
            tracker.is_tracking = false;
            println!("Tracciamento fallito. Reset.");
        }
    }
    false
}
