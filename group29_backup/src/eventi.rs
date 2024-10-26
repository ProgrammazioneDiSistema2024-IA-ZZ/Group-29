
use std::cmp::PartialEq;
use rdev::{listen, Event, EventType};  // Importa rdev per ascoltare gli eventi globali del mouse
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

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
pub fn check_movement(screen_width: f64, screen_height: f64, stop_flag: Arc<Mutex<bool>>) -> bool{
    let tolerance = 50.0; // Tolleranza di 5 pixel
    let mut is_rectangle = false;
    let mut prev_x = 0.0;
    let mut prev_y = 0.0;
    let mut current_border = Border::None;
    let mut direction = Direction::Unknown;
    let mut corner_reached = false;
    let mut initial_x=0.0;;
    let mut initial_y=0.0;
    let mut initial_corner = Corner::None;
    let mut flag_fine = false;
    let mut rectangle_completed = false;


    // Ascolta eventi del mouse con rdev

    if let Err(err) = listen(move |event: Event| {

        if *stop_flag.lock().unwrap() {
            println!("Stop flag is set, terminating check_movement...");
            return;
        }

        if let EventType::MouseMove { x, y } = event.event_type {
            let mut corner = is_in_corner(x, y, screen_width, screen_height);

            if corner != Corner::None && !corner_reached {
                // Primo movimento in un angolo
                is_rectangle = true;
                initial_x = x;
                initial_y = y;
                prev_x = x;
                prev_y = y;
                corner_reached = true;
                initial_corner = corner;
                println!("Mouse in corner {:?} , waiting for direction", initial_corner);

            } else if is_rectangle && direction == Direction::Unknown && corner == Corner::None {
                //Ci entra nel momento in cui non è più nell'intorno dell'angolo e definisce la direzione
                match initial_corner {
                    Corner::TopLeft => {
                        println!(" X:{},Y:{} ", x,y );
                        if y.abs() < tolerance && x.abs() >= tolerance{
                            direction = Direction::Clockwise;
                            current_border = Border::Top;
                        } else if x.abs() < tolerance && y.abs() >= tolerance {
                            direction = Direction::CounterClockwise;
                            current_border = Border::Left;
                        }
                    }
                    Corner::TopRight => {
                        if (x - screen_width ).abs() < tolerance && y.abs() >= tolerance {
                            direction = Direction::Clockwise;
                            current_border = Border::Right;
                        } else if y.abs() < tolerance && (x-screen_width).abs() >= tolerance {
                            direction = Direction::CounterClockwise;
                            current_border = Border::Top;
                        }
                    }
                    Corner::BottomRight => {
                        if (y- screen_height ).abs() < tolerance && (x- screen_width).abs() >= tolerance {
                            direction = Direction::Clockwise;
                            current_border = Border::Bottom;
                        } else if (x - screen_width).abs() < tolerance && (y-screen_height).abs() >= tolerance {
                            direction = Direction::CounterClockwise;
                            current_border = Border::Right;
                        }
                    }
                    Corner::BottomLeft => {
                        if x.abs() < tolerance && (y-screen_height).abs() >= tolerance{
                            direction = Direction::Clockwise;
                            current_border = Border::Left;
                        } else if (y-screen_height).abs() < tolerance && x.abs() >= tolerance{
                            direction = Direction::CounterClockwise;
                            current_border = Border::Bottom;
                        }
                    }
                    Corner::None => {
                        println!("Angolo non trovato");
                    }
                }
                prev_x = x;
                prev_y = y;
            } else if is_rectangle && direction != Direction::Unknown && corner == Corner::None { //Controlli sui bordi e fuori dall'intorno di un angolo

                // Movimento lungo i bordi, controlla i bordi in base alla direzione
                flag_fine=true;//Settata la direzione ,l'unico modo per chiudere il rettangolo è completarlo
                match current_border {
                    Border::Top => {
                        match direction {
                            Direction::Clockwise => {
                                if (y.abs() < tolerance) && ((prev_x <= x) && x < (screen_width as f64)) {
                                    println!("Top border match!({}),({})", x, y);
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    //Siamo fuori dalla possibilita di tracciare un rettangolo
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed top border check ({}),({}), stopping...", x, y);
                                }
                            }
                            Direction::CounterClockwise => {
                                if (y.abs() < tolerance) && ((prev_x >= x) && x >= (0f64)) {
                                    println!("Top border match!({}),({})", x, y);
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed top border check ({}),({}), stopping...", x, y);
                                }
                            }
                            Direction::Unknown => {
                                println!("Problema con la direzione!!");
                            }
                        }
                    }
                    Border::Right => {
                        match direction {
                            Direction::Clockwise => {
                                if (x - screen_width as f64).abs() < tolerance && (prev_y <= y) && y < (screen_height as f64) {
                                    println!("Right border match!");
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed right border check, x:{},y:{}", x, y);
                                }
                            }
                            Direction::CounterClockwise => {
                                if (x - screen_width as f64).abs() < tolerance && (prev_y >= y) && y > (0f64) {
                                    println!("Right border match!");
                                    prev_y = y;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed right border check, stopping...");
                                }
                            }
                            Direction::Unknown => {
                                println!("Problema con la direzione!!");
                            }
                        }
                    }
                    Border::Bottom => {
                        match direction {
                            Direction::Clockwise => {
                                if (y - screen_height as f64).abs() < tolerance && (prev_x >= x) && x > 0.0 {
                                    println!("Bottom border match!");
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed bottom border check, x:{},y:{}", x, y);
                                }
                            }
                            Direction::CounterClockwise => {
                                if (y - screen_height as f64).abs() < tolerance && (prev_x <= x) && x < screen_width as f64 {
                                    println!("Bottom border match!");
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed bottom border check, stopping...");
                                }
                            }
                            Direction::Unknown => {
                                println!("Problema con la direzione!!");
                            }
                        }
                    }
                    Border::Left => {
                        match direction {
                            Direction::Clockwise => {
                                if (x.abs() < tolerance) && (prev_y >= y) && y > 0.0 {
                                    println!("Left border match!");
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
                                    println!("Failed left border check, stopping...");
                                }
                            }

                            Direction::CounterClockwise => {
                                if (x.abs() < tolerance) && (prev_y <= y) && y < screen_height as f64 {
                                    println!("Left border match!");
                                    prev_y =y;
                                    prev_x = x;
                                } else {
                                    flag_fine=false;
                                    is_rectangle = false;
                                    direction = Direction::Unknown;
                                    corner_reached = false;
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
            }else {
                // Qui sono nell'intorno dell'angolo -> Switch del bordo
                println!("Mouse in {:?} neighbourhood ({}),({}), {} ,{:?}", corner,x, y,is_rectangle,direction);
                println!("Initial corner is :{:?}",initial_corner);
                if(initial_corner == corner && flag_fine){
                    println!("Rectangle completed!!!!!!");
                    flag_fine=false;
                    rectangle_completed=true;
                    track_minus_sign(screen_width,screen_height,Arc::clone(&stop_flag));
                }else{
                    match direction{
                        Direction::Clockwise =>{
                            match corner{
                                Corner::TopLeft => current_border = Border::Top,
                                Corner::TopRight => current_border = Border::Right,
                                Corner::BottomLeft => current_border = Border::Left,
                                Corner::BottomRight => current_border = Border::Bottom,
                                Corner::None => {}
                            }

                        }
                        Direction::CounterClockwise=>{
                            match corner{
                                Corner::TopLeft => current_border = Border::Left,
                                Corner::TopRight => current_border = Border::Top,
                                Corner::BottomLeft => current_border = Border::Bottom,
                                Corner::BottomRight => current_border = Border::Right,
                                Corner::None => {}
                            }

                        }
                        Direction::Unknown=>{
                            println!("Unknown direction");

                        }
                    }
                }

            }
        }
    }) {
        println!("Errore nell'ascolto degli eventi del mouse: {:?}", err);
    }
    rectangle_completed
}


pub fn track_minus_sign(screen_width: f64, screen_height: f64, stop_flag: Arc<Mutex<bool>>) {
    let tolerance = 50.0;
    let min_length = screen_width as f64 * 0.2; // Lunghezza minima del segno meno
    let mut is_tracking = false;
    let mut initial_x = 0.0;
    let mut initial_y = 0.0;
    let mut prev_x = 0.0;
    let mut is_minus_sign = false;

    // Ciclo di ascolto degli eventi del mouse

    // Questo gestisce il ciclo di ascolto degli eventi
    if let Err(err) = listen(move |event: Event| {

        if *stop_flag.lock().unwrap() {
            println!("Stop flag is set, terminating minus sign...");
            return;
        }

        if let EventType::MouseMove { x, y } = event.event_type {
            if !is_tracking {
                // Inizia a tracciare il segno meno dal primo movimento orizzontale
                initial_x = x;
                initial_y = y;
                prev_x = x;
                is_tracking = true;
                println!("Tracking started at position: ({}, {})", x, y);
            } else {
                // Verifica se il movimento è orizzontale
                if (initial_y - y).abs() < tolerance && (x - prev_x).abs() >= 0.0 {
                    prev_x = x;
                    println!("Tracking minus sign: current position ({}, {})", x, y);

                } else {
                    if (initial_y - y).abs() >= tolerance {
                        // Movimento fuori tolleranza, reset del tracciamento
                        is_tracking = false;
                        println!("Movement out of tolerance. Resetting tracking.");
                    }
                }

                // Controlla se il segno meno è abbastanza lungo
                if (prev_x - initial_x) >= min_length {
                    is_minus_sign = true; // Setta la variabile di stato
                    println!("Minus sign detected successfully!");
                    let mut stop = stop_flag.lock().unwrap();
                    *stop = true;  // Segnala che il tracking è terminato
                    return; // Uscire dal callback, fermando ulteriori elaborazioni
                }
            }
        }
    }) {
        println!("Error while listening to mouse events: {:?}", err);
    }



    // Qui potresti anche gestire eventuali azioni post-tracciamento
    if !is_minus_sign {
        println!("Minus sign tracking failed.");
    }
}