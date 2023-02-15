use std::io::{self, Write};

fn main() {
    println!("discount eligibility checker written in rust.");

    let discount: bool;

    print!("enter age in years: ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line.");
    let age: u8 = input.trim().parse().expect("invalid number.");

    if age >= 60 {
        discount = true;
    } else {
        print!("do you have a student pass (y/N)? ");
        io::stdout().flush().unwrap();
        let mut student_pass: String = String::new();
        io::stdin().read_line(&mut student_pass).expect("failed to read line.");
        let student_pass: char = student_pass.trim().parse().unwrap_or_else(|_| {'n'});

        if student_pass == 'y' || student_pass == 'Y' {
            discount = true;
        } else {
            discount = false;
        }
    }

    if discount {
        println!("you get a 50% discount!");
    } else {
        println!("you get to pay full price!");
    }
}




 