use std::io::{self, Write};

fn main() {
    println!("body mass index calculator written in rust.");

    print!("enter mass in kilograms: ");
    io::stdout().flush().unwrap();
    let mass = read_float();

    print!("enter height in metres: ");
    io::stdout().flush().unwrap();
    let height = read_float();

    let bmi :f64 = mass / (height * height);

    println!("your bmi is {:.2}.", bmi);

    if bmi < 18.5 {
        println!("Possible nutritional deficiency and osteoporosis.")
    } else if bmi < 23.0 {
        println!("Low risk (healthy range).")
    } else if bmi < 27.5 {
        println!("Moderate risk of developing heart disease, high blood pressure, stroke, diabetes mellitus.")
    } else {
        println!("High risk of developing heart disease, high blood pressure, stroke, diabetes mellitus. Metabolic Syndrome.")
    }
}

fn read_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line.");
    let float = input.trim().parse().expect("invalid number.");
    float
}