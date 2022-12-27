use crate::cli::Cli;
use crate::model::{Input, Stage};
use anyhow::Result;
use autopilot::key::{KeyCode, KeyCodeConvertible};
use clap::Parser;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::path::PathBuf;
use std::{thread::sleep, time::Duration};

extern crate autopilot;

mod cli;
mod model;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input = load_input(&cli.file)?;

    for stage in &input.stages {
        print!("Ready to start stage {} ", stage.order);
        stdout().flush()?;

        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        delay(&cli.start_delay)?;
        run_stage(stage, &cli);
    }

    println!("\n\nFinished");

    Ok(())
}

fn run_stage(stage: &Stage, cli: &Cli) {
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

        autopilot::key::type_string(String::from(ch).as_ref(), &[], cli.input_delay, 10.);
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

fn delay(seconds: &u8) -> Result<()> {
    print!("Starting in ");
    for i in 0..*seconds {
        print!("{}... ", seconds - i);
        stdout().flush()?;
        sleep(Duration::from_secs(1));
    }

    println!();

    Ok(())
}

fn load_input(path: &PathBuf) -> Result<Input> {
    let f = File::open(path)?;

    let mut input: Input = serde_yaml::from_reader(BufReader::new(f))?;
    input.stages.sort_by_key(|s| s.order);

    Ok(input)
}
