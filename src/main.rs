#![warn(clippy::all, clippy::pedantic)]

use chrono::{DateTime, Utc};
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use std::fs;
use std::{env, path::PathBuf};

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    println!("Press F8 or PrintScreen to take a screenshot");

    let args: Vec<String> = env::args().collect();

    let screens_dir: String = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path: PathBuf = env::current_dir()?;
    path.push(&screens_dir);

    fs::create_dir_all(path)?;

    if let Err(error) = grab(move |e: Event| callback(e, &screens_dir)) {
        println!("Error: {error:?}");
    }

    Ok(())
}

fn callback(event: Event, screens_dir: &String) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::F8) => {
            make_screen(screens_dir);
            None
        }
        EventType::KeyPress(Key::PrintScreen) => {
            make_screen(screens_dir);
            None
        }
        _ => Some(event),
    }
}

fn make_screen(screens_dir: &String) {
    let screens: Vec<Screen> = Screen::all().unwrap();

    let now: DateTime<Utc> = Utc::now();

    for screen in screens {
        let image = screen.capture().unwrap();

        image
            .save(format!(
                "{}/{}.png",
                screens_dir,
                now.format("%Y-%m-%d-%H-%M-%S-%3f").to_string()
            ))
            .unwrap();
    }
}
