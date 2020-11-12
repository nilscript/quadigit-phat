use chrono::Local;
use ctrlc;
use docopt::Docopt;
use quadigit_phat::{Digit, Dimming, Display, PHat, PHatExt};
use rppal::i2c::{Error, I2c};
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const USAGE: &str = "
Usage: clock [options]

Options:
  -h --help         Show this screen.
  --dimming=<kn>    Set display dimmer [default: 15].
  --format=<fmt>    Set the clock formatter [default: %H%M].
  --no-dot          Disable blinking dot.
  --no-teardown     Don't clean display on SIGHUP.
  --period=<ms>     Set the period of the clock [default: 1000].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dimming: u8,
    flag_format: String,
    flag_no_dot: bool,
    flag_no_teardown: bool,
    flag_period: u64,
}

fn main() -> Result<(), Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    eprintln!("{:?}", args);

    eprintln!("Setting up display...");
    let mut phat = PHat::new(I2c::new()?, 112u8);
    phat.initialize()?;
    phat.set_dimming(Dimming::from_u8(args.flag_dimming).unwrap())?;
    phat.set_display(Display::ON)?;

    eprintln!("Setting up termination handler...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        eprintln!("Stopping clock...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting termination handler");

    eprintln!("Started clock");
    while running.load(Ordering::SeqCst) {
        let time = Local::now();

        // Ascii-string representing now
        let now = time.format(&args.flag_format).to_string();

        phat.print(&*now)?;
        
        // Every second toggle the middle decimal
        if !args.flag_no_dot {
            phat.set_decimal(Digit::DIGIT_1, time.timestamp() & 1 == 0)?;
        }

        thread::sleep(Duration::from_millis(args.flag_period));
    }
    eprintln!("Stopped clock");


    if !args.flag_no_teardown {
        eprintln!("Tearing down display...");
        phat.clear_display_buffer();
        phat.write_display_buffer()?;

        phat.set_display(Display::OFF)?;
        //    phat.set_oscillator(Oscillator::OFF)
    }

    Ok(())
}
