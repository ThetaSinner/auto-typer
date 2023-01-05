pub use crate::config::Config;
use crate::model::{Input, Stage};
use anyhow::Result;
use autopilot::key::{type_string, Code, Flag, KeyCode};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

pub mod config;
pub mod model;

pub fn load_input(config: &Config) -> Result<Input> {
    let f = File::open(&config.file)?;

    let mut input: Input = serde_yaml::from_reader(BufReader::new(f))?;
    input.stages.sort_by_key(|s| s.order);

    Ok(input)
}

pub fn run_stage(stage: &Stage, config: &Config) {
    let input = &stage.input;

    let mut input_chars = input.chars().peekable();

    while let Some(ch) = input_chars.next() {
        if ch == '\n' {
            continue;
        }

        if ch == '%' && Some(&'(') == input_chars.peek() {
            input_chars.next();
            let mut control = String::new();
            let mut ended = false;
            while let Some(ch) = input_chars.next() {
                if ch == ')' {
                    ended = true;
                    break;
                }

                control.push(ch);
            }

            if !ended {
                panic!("control sequence not terminated");
            }

            apply_control(control);
            continue;
        }

        type_string(String::from(ch).as_ref(), &[], config.wpm, 10.);
    }
}

fn apply_control(control: String) {
    let mut parts = control.split('-');
    let action = parts.next().unwrap();
    let repeat = parts.next().unwrap_or("1").parse::<usize>().unwrap();
    for _ in 0..repeat {
        match action {
            "n" => tap(KeyCode::Return),
            "home" => tap(KeyCode::Home),
            "end" => tap(KeyCode::End),
            "up" => tap(KeyCode::UpArrow),
            "down" => tap(KeyCode::DownArrow),
            "left" => tap(KeyCode::LeftArrow),
            "right" => tap(KeyCode::RightArrow),
            "del_back" => tap(KeyCode::Backspace),
            "select_to_start" => tap_with_modifiers(KeyCode::Home, &[Flag::Shift]),
            "select_to_end" => tap_with_modifiers(KeyCode::End, &[Flag::Shift]),
            #[cfg(target_os = "macos")]
            "copy" => type_string("c", &[Flag::Meta], 60., 0.),
            #[cfg(not(target_os = "macos"))]
            "copy" => type_string("c", &[Flag::Control], 60., 0.),
            #[cfg(target_os = "macos")]
            "paste" => type_string("v", &[Flag::Meta], 60., 0.),
            #[cfg(not(target_os = "macos"))]
            "paste" => type_string("v", &[Flag::Control], 60., 0.),
            _ => panic!("invalid control - {}", control),
        };
        // Short sleep to avoid missing key events
        sleep(Duration::from_millis(5));
    }

    // Delay after the action so that repeated actions are fast but others are visible
    sleep(Duration::from_millis(100));
}

fn tap(key: KeyCode) {
    tap_with_modifiers(key, &[]);
}

fn tap_with_modifiers(key: KeyCode, flags: &[Flag]) {
    autopilot::key::tap(&Code(key), flags, 0, 0);
}
