use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Instant;
use rdev::{listen, Event};
use rfd::MessageDialog;

use crate::suoni::play_sound_sign;
use crate::tracker::{RectangleTracker, MinusSignTracker, track_rectangle, detect_minus_sign, DEBOUNCE_INTERVAL};

/// Funzione principale per monitorare il movimento rettangolare e passare al tracciamento del segno meno
pub fn monitor_movement(screen_width: f64, screen_height: f64, done_flag: Arc<AtomicBool>, done_sender: mpsc::Sender<()>) {
    let mut tracker_rectangle = RectangleTracker::new();
    let mut tracker_minus_sign = MinusSignTracker::new();
    let done_flag_clone = Arc::clone(&done_flag);
    let last_event_time = Arc::new(Mutex::new(Instant::now()));
    let mut monitoring_rectangle = true;

    println!("Monitoraggio movimento avviato.");


    listen(move |event: Event| {
        if done_flag.load(Ordering::Relaxed) {
            return;
        }

        // Limita la frequenza degli eventi (debounce)
        let now = Instant::now();
        let mut last_time = last_event_time.lock().unwrap();
        if now.duration_since(*last_time) < DEBOUNCE_INTERVAL {
            return;
        }
        *last_time = now;

        if monitoring_rectangle {
            // Monitoraggio del rettangolo
            if track_rectangle(&mut tracker_rectangle, screen_width, screen_height, event) {
                println!("Rettangolo completato!");

                if let Err(e) = play_sound_sign() {
                    eprintln!("Errore durante la riproduzione del suono: {}", e);
                }
                // Passa al monitoraggio del segno meno
                monitoring_rectangle = false;
                tracker_minus_sign = MinusSignTracker::new(); // Resetta il tracker
            }
        } else {
            // Monitoraggio del segno meno
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



















/* 
    listen(move |event: Event| {
        if done_flag_clone.load(Ordering::Relaxed) {
            return;
        }

        let now = Instant::now();
        let mut last_time = last_event_time.lock().unwrap();

        if now.duration_since(*last_time) < DEBOUNCE_INTERVAL {
            return;
        }
        *last_time = now;

        if track_rectangle(&mut tracker, screen_width, screen_height, event) {
            println!("Rettangolo completato!");

            if let Err(e) = play_sound_sign() {
                eprintln!("Errore durante la riproduzione del suono: {}", e);
            }


            let done_flag_clone2 = Arc::clone(&done_flag);
            let done_sender_clone = done_sender.clone();
            thread::spawn(move || {
                monitor_minus_sign(screen_width, screen_height, done_flag_clone2, done_sender_clone);
            });
        }
    })
    .expect("Errore durante l'ascolto degli eventi.");
}
*/

/// Avvia il monitoraggio per rilevare un segno meno
pub fn monitor_minus_sign(screen_width: f64, screen_height: f64, done_flag: Arc<AtomicBool>, done_sender: mpsc::Sender<()>) {
    let mut tracker = MinusSignTracker::new();

    listen(move |event: Event| {
        if done_flag.load(Ordering::Relaxed) {
            return;
        }

        if detect_minus_sign(&mut tracker, screen_width, screen_height, event) {
            done_sender.send(()).expect("Errore nell'invio del segnale di completamento.");
            done_flag.store(true, Ordering::Relaxed);
        }
    })
    .expect("Errore durante l'ascolto degli eventi per il segno meno.");
}
