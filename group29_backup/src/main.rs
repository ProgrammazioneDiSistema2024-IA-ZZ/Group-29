mod eventi;
mod backup;

fn main() {
    eventi::get_screen_dimensions();
    eventi::handle_mouse_events();
}