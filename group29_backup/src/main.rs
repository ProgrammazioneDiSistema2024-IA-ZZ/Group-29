use std::thread;
use crate::eventi::check_movement;

mod eventi;
mod backup;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new(); // Creiamo un event loop temporaneo
    let monitor = event_loop.primary_monitor().unwrap();
    let screen_size = monitor.size();
    let screen_width = screen_size.width;
    let screen_height = screen_size.height;

    println!("Screen size: {}x{}", screen_width, screen_height);

    thread::spawn(move|| {
        check_movement(screen_width as f64,screen_height as f64);
    });

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait; // Aspetta gli eventi
    });

}
