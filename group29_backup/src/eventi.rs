#[cfg(target_os="linux")]
use std::process::Command;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use rdev::{listen, Event};
use native_dialog::MessageDialog;
use crate::suoni::play_sound_sign;
use crate::tracker::{RectangleTracker, MinusSignTracker, track_rectangle, detect_minus_sign, DEBOUNCE_INTERVAL};
use std::thread;

fn show_info_message() {
    thread::spawn(|| {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = MessageDialog::new()
                .set_title("Rettangolo Rilevato")
                .set_type(native_dialog::MessageType::Info)
                .set_text("Rettangolo completato! Ora attendiamo il segno meno.")
                .show_alert(); // Nessun pulsante richiesto
        }

        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("zenity")
                .arg("--info")
                .arg("--text=Rettangolo completato! Ora attendiamo il segno meno (hai 10 secondi per confermare).")
                .arg("--timeout=5") // Chiude automaticamente dopo 5 secondi
                .output();
        }
    });
}

fn handle_rectangle_detection(
    screen_width: f64,
    screen_height: f64,
    event: &Event,
    tracker_rectangle: &mut RectangleTracker,
    tracker_minus_sign: &mut MinusSignTracker,
    monitoring_rectangle: &mut bool,
    timeout_flag: &Arc<AtomicBool>,
) {
    if track_rectangle(tracker_rectangle, screen_width, screen_height, event.clone()) {
        println!("Rettangolo completato!");

        if let Err(e) = play_sound_sign() {
            eprintln!("Errore durante la riproduzione del suono: {}", e);
        }

        // Mostra il messaggio informativo senza pulsanti
        show_info_message();

        // Passa al rilevamento del segno meno
        *monitoring_rectangle = false;
        *tracker_minus_sign = MinusSignTracker::new();

        // Imposta un timeout per il segno meno
        let timeout_flag_clone = Arc::clone(timeout_flag);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            timeout_flag_clone.store(true, Ordering::Relaxed);
        });
    }
}

fn handle_minus_sign_detection(
    screen_width: f64,
    screen_height: f64,
    event: &Event,
    tracker_minus_sign: &mut MinusSignTracker,
    done_sender: &mpsc::Sender<()>,
    done_flag: &Arc<AtomicBool>,
    monitoring_rectangle: &mut bool,
    timeout_flag: &Arc<AtomicBool>,
) {
    if detect_minus_sign(tracker_minus_sign, screen_width, screen_height, event.clone()) {
        println!("Segno meno rilevato!");

        if done_sender.send(()).is_err() {
            eprintln!("Errore nell'invio del segnale di completamento.");
        }
        done_flag.store(true, Ordering::Relaxed);
    } else if timeout_flag.load(Ordering::Relaxed) {
        println!("Timeout: Segno meno non rilevato entro 10 secondi. Riaspettando il rettangolo.");

        // Reimposta per il prossimo ciclo
        timeout_flag.store(false, Ordering::Relaxed);
        *monitoring_rectangle = true;
    }
}

pub fn monitor_movement(screen_width: f64, screen_height: f64, done_flag: Arc<AtomicBool>, done_sender: mpsc::Sender<()>) {
    let mut tracker_rectangle = RectangleTracker::new();
    let mut tracker_minus_sign = MinusSignTracker::new();
    let last_event_time = Arc::new(Mutex::new(Instant::now()));
    let mut monitoring_rectangle = true;
    let timeout_flag = Arc::new(AtomicBool::new(false));

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
            handle_rectangle_detection(
                screen_width,
                screen_height,
                &event,
                &mut tracker_rectangle,
                &mut tracker_minus_sign,
                &mut monitoring_rectangle,
                &timeout_flag,
            );
        } else {
            handle_minus_sign_detection(
                screen_width,
                screen_height,
                &event,
                &mut tracker_minus_sign,
                &done_sender,
                &done_flag,
                &mut monitoring_rectangle,
                &timeout_flag,
            );
        }
    })
    .expect("Errore durante l'ascolto degli eventi.");
}
