use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use rdev::{listen, Event};
use native_dialog::MessageDialog;
use crate::suoni::play_sound_sign;
use crate::tracker::{RectangleTracker, MinusSignTracker, track_rectangle, detect_minus_sign, DEBOUNCE_INTERVAL};
use std::process::Command;
use std::thread;

fn show_confirmation_dialog() -> bool {
    // Eseguiamo il dialogo in un thread separato
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        #[cfg(not(target_os = "linux"))]
        {
            let result = MessageDialog::new()
                .set_title("Rettangolo Rilevato")
                .set_type(native_dialog::MessageType::Info)
                .set_text("Rettangolo completato! Vuoi continuare a rilevare il segno meno oppure ricominciare?")
                .show_confirm(); // True for "Yes", false for "No"

            tx.send(result.is_ok() && result.unwrap()).unwrap();
        }

        #[cfg(target_os = "linux")]
        {
            let output = Command::new("zenity")
                .arg("--question")
                .arg("--text=Rettangolo completato! Vuoi continuare a rilevare il segno meno oppure ricominciare?")
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    tx.send(output.stdout.contains(b"Yes")).unwrap();
                } else {
                    eprintln!("Errore durante l'esecuzione di zenity");
                    tx.send(false).unwrap();
                }
            } else {
                eprintln!("Errore nell'esecuzione del comando zenity.");
                tx.send(false).unwrap();
            }
        }
    });

    // Restituiamo il risultato della conferma
    rx.recv().unwrap()
}

fn handle_rectangle_detection(
    screen_width: f64,
    screen_height: f64,
    event: &Event,
    tracker_rectangle: &mut RectangleTracker,
    tracker_minus_sign: &mut MinusSignTracker,
    monitoring_rectangle: &mut bool,
) {
    if track_rectangle(tracker_rectangle, screen_width, screen_height, event.clone()) {
        println!("Rettangolo completato!");

        if let Err(e) = play_sound_sign() {
            eprintln!("Errore durante la riproduzione del suono: {}", e);
        }

        // Show dialog and decide whether to continue or restart
        if show_confirmation_dialog() {
            *monitoring_rectangle = false;
            *tracker_minus_sign = MinusSignTracker::new();
        } else {
            *monitoring_rectangle = true;
            *tracker_rectangle = RectangleTracker::new(); // Restart rectangle tracking
        }
    }
}

fn handle_minus_sign_detection(
    screen_width: f64,
    screen_height: f64,
    event: &Event,
    tracker_minus_sign: &mut MinusSignTracker,
    done_sender: &mpsc::Sender<()> ,
    done_flag: &Arc<AtomicBool>,
) {
    if detect_minus_sign(tracker_minus_sign, screen_width, screen_height, event.clone()) {
        println!("Segno meno rilevato!");

        if done_sender.send(()).is_err() {
            eprintln!("Errore nell'invio del segnale di completamento.");
        }
        done_flag.store(true, Ordering::Relaxed);
    }
}

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
            handle_rectangle_detection(screen_width, screen_height, &event, &mut tracker_rectangle, &mut tracker_minus_sign, &mut monitoring_rectangle);
        } else {
            handle_minus_sign_detection(screen_width, screen_height, &event, &mut tracker_minus_sign, &done_sender, &done_flag);
        }
    })
    .expect("Errore durante l'ascolto degli eventi.");
}
