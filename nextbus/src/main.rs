use std::process;
use clap::Parser;

use nextbus::{Input, run};

fn main() {
    if let Err(e) = run(Input::parse()) {
        eprintln!("application error: {e}.");
        process::exit(1);
    }
}
