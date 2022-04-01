use chrono::Local;
use docopt::Docopt;
use quadigit_phat::*;
use rppal::i2c::{Error, I2c};
use schedule_recv::periodic;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const USAGE: &str = "
Usage: clock [options]

Options:
  -h --help         Show this screen.
  --dimming=<kn>    Set display dimmer [default: 16].
  --format=<fmt>    Set the clock formatter [default: %H%M].
  --no-dot          Disable blinking dot.
";

#[derive(Deserialize)]
struct Args {
    flag_dimming: u8,
    flag_format: String,
    flag_no_dot: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if !(1..=16).contains(&args.flag_dimming) {
        panic!("Flag dimming is out of bounds. Was {}. Bounds 1..=16", args.flag_dimming);
    }

    eprint!("Setting up display...");
    let mut phat: PHat<I2c, Error>;
    phat = PHat::new(HT16K33::new(I2c::new().unwrap(), 112u8)).unwrap();
    let dimming = DimmingSet::from_u8(args.flag_dimming - 1).unwrap();
    phat.ht16k33_mut().write_dimming_set(dimming).unwrap();
    eprintln!("done");

    eprint!("Setting up termination handler...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        eprint!("Stopping clock...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting termination handler");
    eprintln!("done");

    let mut dot: bool = false;
    let ticktock = periodic(Duration::from_secs(1));

    eprintln!("Started clock");
    while running.load(Ordering::SeqCst) {
        let ascii_now = Local::now().format(&args.flag_format).to_string();
        phat.write_str(ascii_now.as_bytes().iter().map(fonts::ascii));
        if !args.flag_no_dot {
            phat.write_dot(CharDataAddressPointer::P1, dot);
        }
        // Every second toggle the middle decimal
        dot = !dot;

        phat.flush().unwrap();
        ticktock.recv().unwrap();
    }
    eprintln!("Stopped clock");
}
