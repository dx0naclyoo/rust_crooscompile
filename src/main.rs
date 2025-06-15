#![warn(clippy::all, clippy::pedantic)]

use chrono::{DateTime, Utc};
use std::env;
use std::fs;
use rdev::{grab, Event, EventType, Key};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::PathBuf;
use xcap::Monitor;


lazy_static! {
    static ref SCREENS_DIR: Mutex<String> = Mutex::new(String::from("/screens/"));
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let screens_dir = args.get(1).unwrap_or(&String::from("/screens/")).to_string();
    let mut path = env::current_dir()?;
    path.push(&screens_dir);
    println!("{args:#?}");
    println!("{path:#?}");
    
    fs::create_dir_all(path)?;
    *SCREENS_DIR.lock().unwrap() = screens_dir.clone();

    if let Err(error ) = grab(callback) {
        println!("grab Error {error:#?}");
    }

    Ok(())
}

fn callback(event: Event) -> Option<Event> {
    let screens_dir: String = SCREENS_DIR.lock().unwrap().clone();

    match event.event_type {
        EventType::KeyPress(Key::PrintScreen) => {
            make_screen(&screens_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screens_dir: &str) {
    println!("Make Screen {screens_dir}");

    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        let now: DateTime<Utc> = Utc::now();

        let monitor_name_result = monitor.name();
        let name = monitor_name_result.as_deref().unwrap_or("unknown");
        let filename = format!(
            "{}-{}.png",
            now.format("%d-%m-%Y_%H_%M_%S"),
            normalized(name)
        );
        let mut full_path = PathBuf::from(screens_dir);
        full_path.push(filename);

        image.save(full_path).unwrap();
    }
}

fn normalized(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        .collect()
}