use std::process;
use clap::Parser;

use minigrep::Input;

fn main() {
    // let input: Input = Input::build(&args).unwrap_or_else(|e| {
    //     eprintln!("problem parsing arguments: {e}.");
    //     process::exit(1);
    // });

    if let Err(e) = minigrep::run(Input::parse()) {
        eprintln!("application error: {e}.");
        process::exit(1);        
    }
}
