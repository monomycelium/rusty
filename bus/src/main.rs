use std::process;
use clap::Parser;

use bus::Input;

fn main() {
    if let Err(e) = bus::run(Input::parse()) {
        eprintln!("application error: {e}.");
        process::exit(1);
    }
}
