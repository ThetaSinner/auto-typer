use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use core::Config;
use std::io::{stdin, stdout, Write};
use std::{thread::sleep, time::Duration};

extern crate core;

mod cli;

fn main() -> Result<()> {
    let config: Config = Cli::parse().into();

    let input = core::load_input(&config)?;

    for stage in &input.stages {
        print!("Ready to start stage {} ", stage.order);
        stdout().flush()?;

        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        delay(&config.start_delay)?;
        core::run_stage(stage, &config);
    }

    println!("\n\nFinished");

    Ok(())
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
