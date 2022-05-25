use chrono::Local;
use docopt::Docopt;
use quadigit_phat::*;
use graceful::SignalGuard;
use rppal::i2c::{Error, I2c};
use schedule_recv::periodic;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;

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

static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if !(1..=16).contains(&args.flag_dimming) {
        panic!("Flag dimming is out of bounds. Was {}. Bounds 1..=16", args.flag_dimming);
    }
 
    eprint!("Setting up display...");
    let mut phat: PHat<I2c, Error>;
    {
        phat = PHat::new(I2c::new().unwrap(), 112u8, fonts::ascii).unwrap();
        phat.power_on().unwrap();

        let dimming = DimmingSet::from_u8(args.flag_dimming - 1).unwrap();
        phat.write_dimming_set(dimming).unwrap();
    } 
    eprintln!("done");

    let sg = SignalGuard::new();
    let ticktock = periodic(Duration::from_secs(1));

    let handle = thread::spawn(move || {
        eprintln!("Started clock");
        while RUNNING.load(Ordering::Acquire) {
            let now = Local::now();
            let ascii_now = now.format(&args.flag_format).to_string();
            phat.write_str(&ascii_now);
            if !args.flag_no_dot && now.timestamp() & 1 == 1 {
                phat.write_dot(CharDataAddressPointer::P1, true);
            }

            phat.flush().unwrap();
            ticktock.recv().unwrap();
        }
        phat.shutdown().unwrap();
        eprintln!("Stopped clock");
    });
    
    sg.at_exit(move |signal| {
        eprint!("Signal {} recieved", signal);
        eprint!("Shutting down");
        RUNNING.store(false, Ordering::Release);
        handle.join().unwrap();
    });
}
