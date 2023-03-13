use std::process::exit;

fn main() {
    if let Err(or) = lewis::run() {
        eprintln!("application error: {}.", or);
        exit(1);
    };
}
