
use std::cmp::PartialEq;
use rdev::{listen, Event, EventType};  // Importa rdev per ascoltare gli eventi globali del mouse
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
fn is_in_corner(x: f64, y: f64, screen_width: u32, screen_height: u32) -> Corner{
    let tolerance = 5.0; // Tolleranza di 5 pixel
    if (x.abs() < tolerance && y.abs() < tolerance) {
        Corner::TopLeft
    } else if ((x - screen_width as f64).abs() < tolerance && y.abs() < tolerance) {
        Corner::TopRight
    } else if (x.abs() < tolerance && (y - screen_height as f64).abs() < tolerance) {
        Corner::BottomLeft
    } else if ((x - screen_width as f64).abs() < tolerance && (y - screen_height as f64).abs() < tolerance) {
        Corner::BottomRight
    } else{
        Corner::None
    }
}
pub fn check_movement(screen_width: u32, screen_height: u32) {
    let tolerance = 80.0; // Tolleranza di 5 pixel
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

    // Ascolta eventi del mouse con rdev
    if let Err(err) = listen(move |event: Event| {
        if let EventType::MouseMove { x, y } = event.event_type {
            let mut corner = is_in_corner(x, y, screen_width, screen_height);
            println!("Primo passo,l'angolo è : {:?}, con coordinate x:{},y{} ", corner, x, y);

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
                //thread::sleep(Duration::from_millis(200));
            } else if is_rectangle && direction == Direction::Unknown /*corner == Corner::None*/ {
                //Ci entra nella seconda iterazione , quando is_rectangle=true e la direzione è settata ancora a Unknown

                //Prende coordinate e verifica il bordo
                //In base al bordo -> definire direzione
                // TopLeft => { y.abs<tolerance -> direction = ClockWise } { x.abs < tolerance -> direction = CounterClockWise}
                match initial_corner {
                    Corner::TopLeft => {
                        if x > prev_x && (y - prev_y).abs() < tolerance {
                            direction = Direction::Clockwise;
                        } else if y < prev_y && (x - prev_x).abs() < tolerance {
                            direction = Direction::CounterClockwise;
                        }
                    }
                    Corner::TopRight => {
                        if (x - prev_x).abs() < tolerance && y > prev_y {
                            direction = Direction::Clockwise;
                        } else if (x - prev_x).abs() < tolerance && y < prev_y {
                            direction = Direction::CounterClockwise;
                        }
                    }
                    Corner::BottomRight => {
                        if x < prev_x && (y - prev_y).abs() < tolerance {
                            direction = Direction::Clockwise;
                        } else if x > prev_x && (y - prev_y).abs() < tolerance {
                            direction = Direction::CounterClockwise;
                        }
                    }
                    Corner::BottomLeft => {
                        if y<prev_y  && ( x - prev_x).abs() < tolerance {
                            direction = Direction::Clockwise;
                        } else if x < prev_x && (y - prev_y).abs() < tolerance {
                            direction = Direction::CounterClockwise;
                        }
                    }
                    Corner::None => {
                        println!("Angolo non trovato");
                    }
                }
                /*match initial_corner{
                    Corner::TopLeft => {
                        if /*x>prev_x || y< prev_y &&*/ (y.abs() < tolerance){
                            direction = Direction::Clockwise;
                        } else if /*prev_x>x|| y>prev_y && */(x.abs() < tolerance) {
                            direction = Direction::CounterClockwise;
                        }
                    }
                    Corner::TopRight => {
                        if /*(x>prev_x || y> prev_y &&*/ (x.abs() < tolerance){
                           direction = Direction:: Clockwise;
                        } else if /*prev_y>y || prev_x>x &&*/ (y.abs() < tolerance){
                            direction = Direction::CounterClockwise;
                        }
                    }
                    Corner::BottomRight => {
                        if /*x<prev_x || y>prev_y &&*/ (y.abs() < tolerance){
                            direction = Direction::Clockwise;
                        }else if /*prev_x<x||y<prev_y &&*/ (x.abs() < tolerance){
                            direction=Direction::CounterClockwise;
                        }
                    }
                    Corner::BottomLeft => {
                        if /*x<prev_x || y<prev_y &&*/ (y.abs() < tolerance){
                            direction = Direction::Clockwise;
                        }else if /*prev_y<y || prev_x<x &&*/ (x.abs() < tolerance){
                            direction = Direction::CounterClockwise;
                        }

                    }
                    Corner::None =>{ println!("Angolo non trovato")}

                }*/


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
                                    println!("Previous x is:{}", prev_x);
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
                                    println!("Previous x is:{}", prev_x);
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
                                println!("Previous_x: {}, x:{}", prev_x,x);
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
                println!("Mouse moved ({}),({}), {} ,{:?}", x, y,is_rectangle,direction);

                if(initial_corner == corner && flag_fine){
                    println!("Rectangle completed!!!!!!");
                    flag_fine=false;
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
                            println!("Direzione non conosciuta, mi è difficile capire quale bordo :-)");

                        }
                    }
                }

            }
        }
    }) {
        println!("Errore nell'ascolto degli eventi del mouse: {:?}", err);
    }
}