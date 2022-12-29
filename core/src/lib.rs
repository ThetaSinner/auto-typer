pub use crate::config::Config;
use crate::model::{Input, Stage};
use anyhow::Result;
use autopilot::key::KeyCode;
use std::fs::File;
use std::io::BufReader;

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

        autopilot::key::type_string(String::from(ch).as_ref(), &[], config.input_delay, 10.);
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
            _ => panic!("invalid control - {}", control),
        };
    }
}

fn tap(key: KeyCode) {
    autopilot::key::tap(&autopilot::key::Code(key), &[], 0, 0);
}
