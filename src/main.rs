use battery::units::ratio::percent;
use clap::Parser;
use std::ops::Not;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

mod args;
mod error;

use args::Args;
use error::Error;

fn main() -> ExitCode {
    let args = Args::parse();

    let first_run = run(&args);

    if args.wait {
        // If the first run encountered an error, then looping endlessly will likely
        // get us nowhere, so we first check if the first run returned false.
        if matches!(first_run, Ok(false)) {
            while matches!(run(&args), Ok(true)).not() {
                sleep(Duration::from_millis(args.interval));
            }
            return ExitCode::SUCCESS;
        }
    }

    match first_run {
        Err(Error::NoBatteries) => {
            eprintln!("No batteries found, unable to continue program execution");
            ExitCode::from(2)
        }
        Err(Error::TooManyBatteries) => {
            eprintln!("More than one battery found, specify the battery ID");
            ExitCode::from(3)
        }
        Err(Error::UnknownBatteryId) => {
            eprintln!("Provided battery id is invalid");
            ExitCode::from(4)
        }
        Err(Error::Unknown) => {
            eprintln!("Unknown error");
            ExitCode::from(5)
        }
        Err(Error::Battery(e)) => {
            eprintln!("Unknown battery error: {:?}", e);
            ExitCode::from(6)
        }
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
    }
}

fn run(args: &Args) -> Result<bool, Error> {
    let manager = battery::Manager::new()?;

    if args.list {
        for (idx, battery) in manager.batteries()?.enumerate() {
            if battery.is_err() {
                continue;
            }
            println!(
                "Battery {}\t Vendor: {}",
                idx,
                battery.as_ref().unwrap().vendor().unwrap_or("Unknown")
            );
            println!(
                "\t\t model: {}",
                battery.as_ref().unwrap().model().unwrap_or("Unknown")
            );
            println!()
        }
        return Ok(true);
    }

    let battery_count = manager.batteries()?.collect::<Vec<_>>().len();

    if battery_count == 0 {
        return Err(Error::NoBatteries);
    }

    if battery_count > 1 && args.id.is_none() {
        return Err(Error::TooManyBatteries);
    }

    if battery_count > 1 && args.id.unwrap() > battery_count {
        return Err(Error::UnknownBatteryId);
    }

    let battery = manager
        .batteries()?
        .nth(args.id.unwrap_or(0))
        .ok_or(Error::Unknown)??;
    let percentage = battery.state_of_charge().get::<percent>();
    let battery_state = battery.state();

    if args.verbose {
        println!("State:\t{}", battery_state);
        println!("Charge:\t{}%", percentage);
    }

    if let Some(less_than) = args.lt {
        if (percentage < less_than.into()).not() {
            if args.verbose {
                println!("Less than condition failed");
            }
            return Ok(false);
        }
        if args.verbose {
            println!("Less than condition passed");
        }
    }

    if let Some(greater_than) = args.gt {
        if (percentage > greater_than.into()).not() {
            if args.verbose {
                println!("Greater than condition failed");
            }
            return Ok(false);
        }
        if args.verbose {
            println!("Greater than condition passed");
        }
    }

    if let Some(wanted_state) = args.state {
        if battery_state != wanted_state {
            if args.verbose {
                println!("State condition failed");
            }
            return Ok(false);
        }
        if args.verbose {
            println!("State condition passed");
        }
    }

    if let Some(wanted_not_state) = args.not_state {
        if battery_state == wanted_not_state {
            if args.verbose {
                println!("Not state condition failed");
            }
            return Ok(false);
        }
        if args.verbose {
            println!("Not state condition passed");
        }
    }

    if args.verbose {
        println!("All conditions pass, Exiting");
    }

    Ok(true)
}
