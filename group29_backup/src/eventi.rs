
use std::cmp::PartialEq;
use rdev::{listen, Event, EventType};  // Importa rdev per ascoltare gli eventi globali del mouse
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use std::sync::mpsc::Sender; // Importa Sender
use std::sync::atomic::{Ordering,AtomicBool};


#[derive(Debug)]
struct RectangleTracker {
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
}

impl RectangleTracker {
    // Metodo per creare un nuovo RectangleTracker con valori predefiniti
    fn new() -> Self {
        RectangleTracker {
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
        }
    }
}


#[derive(Debug)]
struct TrackingMinusStatus {
    is_tracking: bool,
    initial_x: f64,
    initial_y: f64,
    prev_x: f64,
    is_minus_sign: bool,
}

impl TrackingMinusStatus {
    // Metodo per creare una nuova istanza con valori iniziali
    fn new() -> Self {
        TrackingMinusStatus {
            is_tracking: false,
            initial_x: 0.0,
            initial_y: 0.0,
            prev_x: 0.0,
            is_minus_sign: false,
        }
    }
}

#[derive(Debug)]
enum Border {
    None,
    Top,
    Right,
    Bottom,
    Left,
}

//Angoli
#[derive(PartialEq, Debug)]
enum Corner {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}


#[derive(PartialEq,Debug)]
enum Direction {
    Unknown,
    Clockwise,       // Movimento in senso orario
    CounterClockwise, // Movimento in senso antiorario
}

// Funzione per verificare se il mouse è in un angolo dello schermo
fn is_in_corner(x: f64, y: f64, screen_width: f64, screen_height: f64) -> Corner{
    let tolerance = 50.0; // Tolleranza di 5 pixel
    if (x.abs() < tolerance && y.abs() < tolerance) {
        Corner::TopLeft
    } else if ((x - screen_width).abs() < tolerance && y.abs() < tolerance) {
        Corner::TopRight
    } else if (x.abs() < tolerance && (y - screen_height).abs() < tolerance) {
        Corner::BottomLeft
    } else if ((x - screen_width).abs() < tolerance && (y - screen_height).abs() < tolerance) {
        Corner::BottomRight
    } else{
        Corner::None
    }
}
pub fn check_movement(screen_width: f64, screen_height: f64, done_flag: Arc<AtomicBool>, done_sender: mpsc::Sender<()>) {

    //variabili globali
    let mut tracker = RectangleTracker::new();

    //Flag per terminare il listen
    let done_flag_clone = Arc::clone(&done_flag);

    thread::spawn(move || {
        // Ascolta eventi del mouse con rdev
        listen(move |event: Event| {

            // Controlla se il flag di terminazione è stato impostato
            if done_flag_clone.load(Ordering::Relaxed) {
                return; // Esce dal callback senza fare nulla
            }

            if traccia_rettangolo(&mut tracker,screen_width,screen_height,event){
                println!("Sei dentro traccia rettangolo -> true");
                let done_flag_clone2 = Arc::clone(&done_flag);
                let done_sender_clone = done_sender.clone();
                thread::spawn(move|| {
                    track_minus_sign(screen_width,screen_height,done_flag_clone2, done_sender_clone)
                });
            }

        }).expect("Errore nell'ascolto degli eventi di rdev in check movement");
    });
}


pub fn track_minus_sign(screen_width:f64,screen_height:f64,done_flag: Arc<AtomicBool>, done_sender: mpsc::Sender<()>){

    let mut tracker = TrackingMinusStatus::new();

   listen(move |event:Event| {

       println!("Sei dentro il listen del thread spawn del track_minus_sign e done_flag:{:?}", done_flag);
       if done_flag.load(Ordering::Relaxed){
           return;
       }
       if rileva_segno_meno(&mut tracker,screen_width,screen_height,event) {
           done_sender.send(()).expect("Errore nell'invio del segnale di completmento");
           done_flag.store(true,Ordering::Relaxed);
       }
       }).expect("Errore nell'ascolto degli eventi di rdev in trck minus");
}



pub fn traccia_rettangolo( tracker : &mut RectangleTracker, screen_width: f64, screen_height:f64, event:Event)-> bool{


    let tolerance = 50.0; // Tolleranza di 5 pixel

    if let EventType::MouseMove { x, y } = event.event_type {
        let mut corner = is_in_corner(x, y, screen_width, screen_height);
        if corner != Corner::None && !tracker.corner_reached {
            // Primo movimento in un angolo
            tracker.is_rectangle = true;
            tracker.initial_x = x;
            tracker.initial_y = y;
            tracker.prev_x = x;
            tracker.prev_y = y;
            tracker.corner_reached = true;
            tracker.initial_corner = corner;
            println!("Mouse in corner {:?} , waiting for direction", tracker.initial_corner);
        } else if tracker.is_rectangle && tracker.direction == Direction::Unknown && corner == Corner::None {
            //Ci entra nel momento in cui non è più nell'intorno dell'angolo e definisce la direzione
            match tracker.initial_corner {
                Corner::TopLeft => {
                    println!(" X:{},Y:{} ", x, y);
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
                Corner::None => {
                    println!("Angolo non trovato");
                }
            }
            tracker.prev_x = x;
            tracker.prev_y = y;
        } else if tracker.is_rectangle && tracker.direction != Direction::Unknown && corner == Corner::None { //Controlli sui bordi e fuori dall'intorno di un angolo

            // Movimento lungo i bordi, controlla i bordi in base alla direzione
            tracker.flag_fine = true;//Settata la direzione ,l'unico modo per chiudere il rettangolo è completarlo
            match tracker.current_border {
                Border::Top => {
                    match tracker.direction {
                        Direction::Clockwise => {
                            if (y.abs() < tolerance) && ((tracker.prev_x <= x) && x < (screen_width as f64)) {
                                println!("Top border match!({}),({})", x, y);
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                //Siamo fuori dalla possibilita di tracciare un rettangolo
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed top border check ({}),({}), stopping...", x, y);
                            }
                        }
                        Direction::CounterClockwise => {
                            if (y.abs() < tolerance) && ((tracker.prev_x >= x) && x >= (0f64)) {
                                println!("Top border match!({}),({})", x, y);
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed top border check ({}),({}), stopping...", x, y);
                            }
                        }
                        Direction::Unknown => {
                            println!("Problema con la direzione!!");
                        }
                    }
                }
                Border::Right => {
                    match tracker.direction {
                        Direction::Clockwise => {
                            if (x - screen_width as f64).abs() < tolerance && (tracker.prev_y <= y) && y < (screen_height as f64) {
                                println!("Right border match!");
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed right border check, x:{},y:{}", x, y);
                            }
                        }
                        Direction::CounterClockwise => {
                            if (x - screen_width as f64).abs() < tolerance && (tracker.prev_y >= y) && y > (0f64) {
                                println!("Right border match!");
                                tracker.prev_y = y;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed right border check, stopping...");
                            }
                        }
                        Direction::Unknown => {
                            println!("Problema con la direzione!!");
                        }
                    }
                }
                Border::Bottom => {
                    match tracker.direction {
                        Direction::Clockwise => {
                            if (y - screen_height as f64).abs() < tolerance && (tracker.prev_x >= x) && x > 0.0 {
                                println!("Bottom border match!");
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed bottom border check, x:{},y:{}", x, y);
                            }
                        }
                        Direction::CounterClockwise => {
                            if (y - screen_height as f64).abs() < tolerance && (tracker.prev_x <= x) && x < screen_width as f64 {
                                println!("Bottom border match!");
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed bottom border check, stopping...");
                            }
                        }
                        Direction::Unknown => {
                            println!("Problema con la direzione!!");
                        }
                    }
                }
                Border::Left => {
                    match tracker.direction {
                        Direction::Clockwise => {
                            if (x.abs() < tolerance) && (tracker.prev_y >= y) && y > 0.0 {
                                println!("Left border match!");
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed left border check, stopping...");
                            }
                        }

                        Direction::CounterClockwise => {
                            if (x.abs() < tolerance) && (tracker.prev_y <= y) && y < screen_height as f64 {
                                println!("Left border match!");
                                tracker.prev_y = y;
                                tracker.prev_x = x;
                            } else {
                                tracker.flag_fine = false;
                                tracker.is_rectangle = false;
                                tracker.direction = Direction::Unknown;
                                tracker.corner_reached = false;
                                println!("Failed left border check, stopping...");
                            }
                        }
                        Direction::Unknown => {
                            println!("Problema con la direzione!!");
                        }
                    }
                }
                Border::None => {
                    println!("Not on a valid border");
                }
            }
        } else {
            // Qui sono nell'intorno dell'angolo -> Switch del bordo
            println!("Mouse in {:?} neighbourhood ({}),({}), {} ,{:?}", corner, x, y, tracker.is_rectangle, tracker.direction);
            println!("Initial corner is :{:?}", tracker.initial_corner);
            if (tracker.initial_corner == corner && tracker.flag_fine) {
                println!("Rectangle completed!!!!!!");
                tracker.flag_fine = false;
                return true;
            } else {
                match tracker.direction {
                    Direction::Clockwise => {
                        match corner {
                            Corner::TopLeft => tracker.current_border = Border::Top,
                            Corner::TopRight => tracker.current_border = Border::Right,
                            Corner::BottomLeft => tracker.current_border = Border::Left,
                            Corner::BottomRight => tracker.current_border = Border::Bottom,
                            Corner::None => {}
                        }
                    }
                    Direction::CounterClockwise => {
                        match corner {
                            Corner::TopLeft => tracker.current_border = Border::Left,
                            Corner::TopRight => tracker.current_border = Border::Top,
                            Corner::BottomLeft => tracker.current_border = Border::Bottom,
                            Corner::BottomRight => tracker.current_border = Border::Right,
                            Corner::None => {}
                        }
                    }
                    Direction::Unknown => {
                        println!("Unknown direction");
                    }
                }
            }
        }
    }
    return false;
}

pub fn rileva_segno_meno(tracker: &mut TrackingMinusStatus , screen_width:f64,screen_height:f64,event: Event) ->bool {

    println!("Sei dentro il rilevamento segno meno");
    let tolerance = 50.0;
    let min_length = screen_width as f64 * 0.2; // Lunghezza minima del segno meno

    // Questo gestisce il ciclo di ascolto degli eventi

    if let EventType::MouseMove { x, y } = event.event_type {

        if !tracker.is_tracking {
            // Inizia a tracciare il segno meno dal primo movimento orizzontale
            tracker.initial_x = x;
            tracker.initial_y = y;
            tracker.prev_x = x;
            tracker.is_tracking = true;
            println!("Tracking started at position: ({}, {})", x, y);
        } else {
            // Verifica se il movimento è orizzontale
            if (tracker.initial_y - y).abs() < tolerance && (x - tracker.prev_x).abs() >= 0.0 {
                tracker.prev_x = x;
                println!("Tracking minus sign: current position ({}, {})", x, y);
                if (tracker.prev_x - tracker.initial_x) >= min_length {
                    tracker.is_minus_sign = true; // Setta la variabile di stato
                    return true; // You can use return to exit the closure
                }

            } else {
                if (tracker.initial_y - y).abs() >= tolerance {
                    // Movimento fuori tolleranza, reset del tracciamento
                    tracker.is_tracking = false;
                    println!("Movement out of tolerance. Resetting tracking.");
                }
            }

            // Controlla se il segno meno è abbastanza lungo
            if (tracker.prev_x - tracker.initial_x) >= min_length {
                tracker.is_minus_sign = true; // Setta la variabile di stato
                println!("Minus sign detected successfully!");
                return true; // Puoi usare return per uscire dalla closure
            }
        }
    }
    return false;
}