use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use rdev::{listen, Event};


use crate::suoni::play_sound_sign;
use crate::tracker::{RectangleTracker, MinusSignTracker, track_rectangle, detect_minus_sign, DEBOUNCE_INTERVAL};

pub fn monitor_movement(screen_width: f64, screen_height: f64, done_flag: Arc<AtomicBool>, done_sender: mpsc::Sender<()>) {
    let mut tracker_rectangle = RectangleTracker::new();
    let mut tracker_minus_sign = MinusSignTracker::new();
    let last_event_time = Arc::new(Mutex::new(Instant::now()));
    let mut monitoring_rectangle = true;

    println!("Monitoraggio movimento avviato.");


    listen(move |event: Event| {
        if done_flag.load(Ordering::Relaxed) {
            return;
        }

        let now = Instant::now();
        let mut last_time = last_event_time.lock().unwrap();
        if now.duration_since(*last_time) < DEBOUNCE_INTERVAL {
            return;
        }
        *last_time = now;

        if monitoring_rectangle {

            if track_rectangle(&mut tracker_rectangle, screen_width, screen_height, event) {
                println!("Rettangolo completato!");

                if let Err(e) = play_sound_sign() {
                    eprintln!("Errore durante la riproduzione del suono: {}", e);
                }

                monitoring_rectangle = false;
                tracker_minus_sign = MinusSignTracker::new();
            }
        } else {

            if detect_minus_sign(&mut tracker_minus_sign, screen_width, screen_height, event) {
                println!("Segno meno rilevato!");

                if done_sender.send(()).is_err() {
                    eprintln!("Errore nell'invio del segnale di completamento.");
                }
                done_flag.store(true, Ordering::Relaxed);
            }
        }
    })
    .expect("Errore durante l'ascolto degli eventi.");

}


