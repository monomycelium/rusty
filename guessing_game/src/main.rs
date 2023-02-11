use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("guess the number!, a game written in rust.");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("enter your guess.");
        io::stdout().flush();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal   => {
                println!("you win!");
                break;
            }
        }
    }
}
