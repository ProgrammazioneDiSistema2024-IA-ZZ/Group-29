mod eventi;
mod backup;

fn main() {
    println!("Avvio del programma di backup...");

    // Inizializza l'ascolto degli eventi del mouse
    eventi::handle_mouse_events();
}
