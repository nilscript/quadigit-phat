use chrono::Local;
use docopt::Docopt;
use quadigit_phat::{*, states::*};
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
  --no-shutdown     Don't clean display on SIGHUP.
  --period=<ms>     Set the period of the clock [default: 1000].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_dimming: u8,
    flag_format: String,
    flag_no_dot: bool,
    flag_no_shutdown: bool,
    flag_period: u64,
}

fn main() -> Result<(), Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    eprintln!("Setting up display...");
    let mut phat = HT16K33::new(I2c::new()?, 112u8);
    phat.power_on()?;
    phat.write_dimming(Pulse::new(args.flag_dimming).unwrap())?;

    eprintln!("Setting up termination handler...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        eprintln!("Stopping clock...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting termination handler");

    eprintln!("Started clock");
    while running.load(Ordering::SeqCst) {
        let time = Local::now();
        let ascii_now = time.format(&args.flag_format).to_string();

        phat.set_text(fonts::ascii, ascii_now.chars());
        
        // Every second toggle the middle decimal
        if !args.flag_no_dot {
            phat.set_dot(Digit::P1, time.timestamp() & 1 == 0);
        }

        phat.write_dbuf()?;

        thread::sleep(Duration::from_millis(args.flag_period));
    } eprintln!("Stopped clock");

    if args.flag_no_shutdown {
        Ok(())
    } else {
        eprintln!("Tearing down display...");
        phat.shutdown()
    }
}
