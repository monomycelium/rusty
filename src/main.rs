/*
fn main() {
    println!("hello, this is a rusty project.");
    // data types
    /*
    let mass: f64 = "42.69".parse().expect("Not a number!");
    println!("{}", mass)

    let tuple: (i32, f64, u8) = (347, 3.142, 2);

    let (x, y, z) = tuple;
    let ay = tuple.0;

    println!("x = {}, y = {}, z = {}. hello, aidan{}.", x, y, z, ay)

    let array = [0, 1, 2, 3, 4];
    // let array: [i32; 5] = [0, 1, 2, 3, 4];

    let repeat = [3; 5];
    println!("array:\t{:?}\nrepeat:\t{:?}", array, repeat);

    println!("array[0] = {}", array[0]);
    */



    // control flow
    /*
    let x = if true { 5 } else { 6 };
    println!("{x}");

    let dry_ice: [i32; 3] = [0, 6, 3];
    for element in dry_ice {
        print!("{element}, ");
    }

    println!("starting in…");
    for x in (1..11).rev() {
        print!("\r{x} ");
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("\nblastoff!");
    */



    // slices
    /*
    let x: String = String::from("lorem ipsum");

    // let one: &str = &x[..5];
    // let two: &str = &x[6..];

    // println!("one: {}, two: {}.", one, two);
    
    fn first_word(s: &String) -> &str {
        let bytes: &[u8] = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
    
        &s[..]
    }

    let first_word = first_word(&x);
    assert_eq!(first_word, String::from("lorem"));
    */



    // structs
    /*
    struct Human {
        name: String,
        gender: char,
        age: u8,
        lovable: bool,
    }

    fn love(person: &Human) {
        if person.lovable {
            println!("it turns out that {} is extremely lovable.", person.name);
    
            if person.age >= 18 && person.gender == 'm' {
                println!("ask him out!");
            }
        } else {
            println!("step out of your delusions, for {} is not as lovely as you thought.", person.name);
        }    
    }

    let mut aden: Human = Human { name: String::from("aden"), gender: 'm', age: 13, lovable: true };

    love(&aden);
    aden.age = 21;
    love(&aden);

    fn define_boyfriend(name: String, age: u8) -> Human {
        Human {
            name,
            gender: 'm',
            age,
            lovable: true,
        }
    }

    let brian: Human = define_boyfriend(String::from("brian"), 14);
    println!("Still looking at the screen, {} asks, \"Want to?\" and I'm a flood in a paper cup.", brian.name);

    let jayden: Human = define_boyfriend(String::from("jayden teng"), 13);
    let lookalike: Human = Human { name: String::from("ohwow"), ..jayden };
    println!("oh, hello there, {}-year-old who looks like {}.", lookalike.age, jayden.name);

    struct Color(i32, i32, i32);
    let black: Color = Color(0, 0, 0);
    println!("{}", black.0);

    // methods
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }

        fn square(size: u32) -> Self {
            Rectangle { width: size, height: size }
        }
    }

    let square_one: Rectangle = Rectangle::square(36);
    println!("area of square one:\t\t{}\nperimeter of square one:\t{}", square_one.area(), square_one.perimeter());
    */
} */

// count the number of `n`s:
/*
use std::io::{self, Write};

fn main() {
    let mut string: String = String::new();
    print!("enter a string:\t");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut string).expect("failed to read line.");

    let mut count: u8 = 0;
    let mut vowels: u8 = 0;

    for char in string.chars() {
        match char {
            'n' => count += 1,
            'a'|'e'|'i'|'o'|'u' => vowels += 1,
            _ => continue,
        };
    }

    println!("`n`s:\t\t{}", count);
    println!("vowels:\t\t{}", vowels);
} */