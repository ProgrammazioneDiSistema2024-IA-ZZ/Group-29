
// 1) Prendere informazioni sulle dimensioni dello schermo-> (screen_width , screen_height)

/* 2) Per ogni movimento del mouse fare questo controllo:
    Check che le coordinate corrispondano con quelle di uno dei 4 angoli:
    - True : is_rectangle= true (ipoteticamente può rappresentare un rettangolo)
      - Al prossimo movimento del mouse devo quindi controllare:
        Check che le coordinate corrispondano con un possibile rettangolo anche in base alla direzione
        (es. supponendo parto da (0,0), posso avere due casi:
          - antiorario -> ascisse rimangono tali e quali ( x =0), e ordinate invece devono rispettare il range ( screen_height <y< y_precedente)
          - orario -> ordinate rimangono tali e quali ( y=0) , e ascisse invece devono rispettare il range ( screen_width <x< x_precedente))
         - True: is_rectangle= true e continua il ciclo con attenzione al cambiamento del controllo quando tocca un altro angolo
         - False: is_rectangle= false e blocca il ciclo , che verrà ripreso solo una volta che ritocca un nuovo angolo
     -False : is_rectangle= false

    3) Attenzione: Necessario associare questi controlli:
      - Top_border -> Ordinate=0 ,  x_precedente < Ascisse < screen_width
      - Right_border -> Ascisse= screen_width , y_precedente < Ordinate < screen_height
      - Bottom_border -> Ordinate = screen_height, x_precedente < Ascisse < screen_width
      - Left_border -> Ascisse = 0 , y_precedente < Ordinate < 0

    4) Una volta che si tocca l'angolo corrispondente all'angolo iniziale e is_rectangle = true si termina il controllo


 */

/*use std::cmp::PartialEq;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
};

enum Border {
    None,
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(PartialEq)]
enum Direction {
    Unknown,
    Clockwise,       // Movimento in senso orario
    CounterClockwise, // Movimento in senso antiorario
}

//Controllo che sia un angolo
fn is_in_corner(x: f64, y: f64, screen_width: u32, screen_height: u32) -> bool {
    (x == 0.0 && y == 0.0) ||                               // Top-left corner
        (x == screen_width as f64 && y == 0.0) ||                // Top-right corner
        (x == 0.0 && y == screen_height as f64) ||               // Bottom-left corner
        (x == screen_width as f64 && y == screen_height as f64)  // Bottom-right corner
}




pub fn check_movement(){
    let mut event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();
    let screen_width = size.width;
    let screen_height = size.height;

    let mut is_rectangle = false;
    let mut prev_x: f64 = 0.0;
    let mut prev_y: f64 = 0.0;
    let mut current_border = Border::None;
    let mut direction = Direction::Unknown; // Direzione non ancora determinata
    let mut corner_reached = false;         // Variabile per sapere se siamo partiti da un angolo

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x;
                    let y = position.y;

                    if is_in_corner(x, y, screen_width, screen_height) {
                        if !corner_reached {
                            // Primo movimento in un angolo
                            is_rectangle = true;
                            prev_x = x;
                            prev_y = y;
                            corner_reached = true; // Indica che abbiamo raggiunto un angolo
                            println!("Mouse in corner, waiting for direction");

                        } else if corner_reached && direction == Direction::Unknown {
                            // Secondo movimento: determinare la direzione

                            if x != prev_x && y == prev_y {
                                // Movimento lungo l'asse x (bordo superiore)
                                direction = Direction::Clockwise;
                                current_border = Border::Top;
                                println!("Clockwise -> Moving to Top Border");
                            } else if y != prev_y && x == prev_x {
                                // Movimento lungo l'asse y (bordo sinistro)
                                direction = Direction::CounterClockwise;
                                current_border = Border::Left;
                                println!("CounterClockwise -> Moving to Left Border");
                            }

                            prev_x = x;
                            prev_y = y;
                        }
                    } else if is_rectangle && direction != Direction::Unknown {
                        // Movimento lungo i bordi, controlla i bordi in base alla direzione

                        match current_border {
                            Border::Top => {
                                if y == 0.0 && prev_x < x && x < screen_width as f64 {
                                    println!("Top border match!");
                                    prev_x = x;
                                } else {
                                    is_rectangle = false;
                                    println!("Failed top border check, stopping...");
                                }

                                // Cambia bordo in base alla direzione
                                if x == screen_width as f64 && y == 0.0 {
                                    current_border = if let Direction::Clockwise = direction {
                                        Border::Right
                                    } else {
                                        Border::Left
                                    };
                                    println!("Switching to {}", match current_border {
                                        Border::Right => "Right Border",
                                        Border::Left => "Left Border",
                                        _ => "Unknown Border",
                                    });
                                }
                            }
                            Border::Right => {
                                if x == screen_width as f64 && prev_y < y && y < screen_height as f64 {
                                    println!("Right border match!");
                                    prev_y = y;
                                } else {
                                    is_rectangle = false;
                                    println!("Failed right border check, stopping...");
                                }

                                if y == screen_height as f64 && x == screen_width as f64 {
                                    current_border = if let Direction::Clockwise = direction {
                                        Border::Bottom
                                    } else {
                                        Border::Top
                                    };
                                    println!("Switching to {}", match current_border {
                                        Border::Bottom => "Bottom Border",
                                        Border::Top => "Top Border",
                                        _ => "Unknown Border",
                                    });
                                }
                            }
                            Border::Bottom => {
                                if y == screen_height as f64 && prev_x > x && x > 0.0 {
                                    println!("Bottom border match!");
                                    prev_x = x;
                                } else {
                                    is_rectangle = false;
                                    println!("Failed bottom border check, stopping...");
                                }

                                if x == 0.0 && y == screen_height as f64 {
                                    current_border = if let Direction::Clockwise = direction {
                                        Border::Left
                                    } else {
                                        Border::Right
                                    };
                                    println!("Switching to {}", match current_border {
                                        Border::Left => "Left Border",
                                        Border::Right => "Right Border",
                                        _ => "Unknown Border",
                                    });
                                }
                            }
                            Border::Left => {
                                if x == 0.0 && prev_y > y && y > 0.0 {
                                    println!("Left border match!");
                                    prev_y = y;
                                } else {
                                    is_rectangle = false;
                                    println!("Failed left border check, stopping...");
                                }

                                if x == 0.0 && y == 0.0 {
                                    println!("Rectangle completed!");
                                    *control_flow = ControlFlow::Exit;
                                }
                            }
                            Border::None => {
                                println!("Not on a valid border");
                            }
                        }
                    } else {
                        println!("Mouse moved, but not in a corner or direction not set.");
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}*/

use rdev::{listen, Event, EventType};  // Importa rdev per ascoltare gli eventi globali del mouse
use std::sync::{Arc, Mutex};


enum Border {
    None,
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(PartialEq)]
enum Direction {
    Unknown,
    Clockwise,       // Movimento in senso orario
    CounterClockwise, // Movimento in senso antiorario
}

// Funzione per verificare se il mouse è in un angolo dello schermo
fn is_in_corner(x: f64, y: f64, screen_width: u32, screen_height: u32) -> bool {
    let tolerance = 5.0; // Tolleranza di 5 pixel
    (x.abs() < tolerance && y.abs() < tolerance) ||                               // Top-left corner
        ((x - screen_width as f64).abs() < tolerance && y.abs() < tolerance) ||   // Top-right corner
        (x.abs() < tolerance && (y - screen_height as f64).abs() < tolerance) ||  // Bottom-left corner
        ((x - screen_width as f64).abs() < tolerance && (y - screen_height as f64).abs() < tolerance)  // Bottom-right corner
}

// Funzione principale per monitorare il movimento del mouse
pub fn check_movement(screen_width: u32, screen_height: u32) {
    let mut is_rectangle = false;
    let mut prev_x = 0.0;
    let mut prev_y = 0.0;
    let mut current_border = Border::None;
    let mut direction = Direction::Unknown;
    let mut corner_reached = false;

    // Ascolta eventi del mouse con rdev
    if let Err(err) = listen(move |event: Event| {
        if let EventType::MouseMove { x, y } = event.event_type {
            if is_in_corner(x, y, screen_width, screen_height) {
                if !corner_reached {
                    // Primo movimento in un angolo
                    is_rectangle = true;
                    prev_x = x;
                    prev_y = y;
                    corner_reached = true;
                    println!("Mouse in corner, waiting for direction");
                } else if corner_reached && direction == Direction::Unknown {
                    // Secondo movimento: determinare la direzione
                    if x != prev_x && y == prev_y {
                        // Movimento lungo l'asse x (bordo superiore)
                        direction = Direction::Clockwise;
                        current_border = Border::Top;
                        println!("Clockwise -> Moving to Top Border");
                    } else if y != prev_y && x == prev_x {
                        // Movimento lungo l'asse y (bordo sinistro)
                        direction = Direction::CounterClockwise;
                        current_border = Border::Left;
                        println!("CounterClockwise -> Moving to Left Border");
                    }

                    prev_x = x;
                    prev_y = y;
                }
            } else if is_rectangle && direction != Direction::Unknown {
                // Movimento lungo i bordi, controlla i bordi in base alla direzione

                match current_border {
                    Border::Top => {
                        if y == 0.0 && prev_x < x && x < screen_width as f64 {
                            println!("Top border match!");
                            prev_x = x;
                        } else {
                            is_rectangle = false;
                            println!("Failed top border check, stopping...");
                        }

                        if x == screen_width as f64 && y == 0.0 {
                            current_border = if let Direction::Clockwise = direction {
                                Border::Right
                            } else {
                                Border::Left
                            };
                            println!("Switching to Right Border");
                        }
                    }
                    Border::Right => {
                        if x == screen_width as f64 && prev_y < y && y < screen_height as f64 {
                            println!("Right border match!");
                            prev_y = y;
                        } else {
                            is_rectangle = false;
                            println!("Failed right border check, stopping...");
                        }

                        if y == screen_height as f64 && x == screen_width as f64 {
                            current_border = if let Direction::Clockwise = direction {
                                Border::Bottom
                            } else {
                                Border::Top
                            };
                            println!("Switching to Bottom Border");
                        }
                    }
                    Border::Bottom => {
                        if y == screen_height as f64 && prev_x > x && x > 0.0 {
                            println!("Bottom border match!");
                            prev_x = x;
                        } else {
                            is_rectangle = false;
                            println!("Failed bottom border check, stopping...");
                        }

                        if x == 0.0 && y == screen_height as f64 {
                            current_border = if let Direction::Clockwise = direction {
                                Border::Left
                            } else {
                                Border::Right
                            };
                            println!("Switching to Left Border");
                        }
                    }
                    Border::Left => {
                        if x == 0.0 && prev_y > y && y > 0.0 {
                            println!("Left border match!");
                            prev_y = y;
                        } else {
                            is_rectangle = false;
                            println!("Failed left border check, stopping...");
                        }

                        if x == 0.0 && y == 0.0 {
                            println!("Rectangle completed!");
                        }
                    }
                    Border::None => {
                        println!("Not on a valid border");
                    }
                }
            } else {
                println!("Mouse moved, but not in a corner or direction not set.");
            }
        }
    }) {
        println!("Errore nell'ascolto degli eventi del mouse: {:?}", err);
    }
}

