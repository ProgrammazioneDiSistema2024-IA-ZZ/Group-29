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

    // Inizia il monitoraggio del movimento del mouse
    check_movement(screen_width, screen_height);
}
