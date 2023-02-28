/*
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
} */

/*
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("hi.");
    }
}

fn main() {
    let m = Message::Write(String::from("hayes"));
    m.call();
} */

/*
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
} */

/*
#[derive(Debug)]
enum State {
    Alaska,
    Georgia,
}

enum Coin {
    Dime,
    Nickel,
    Penny,
    Quarter(State),
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("state of the quarter: {:?}", state);
            25
        },
    }
}

fn main() {
    value(Coin::Quarter(State::Alaska));
} */

/*
fn main() {    
    fn add(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    println!("{:?}", add(five));
    println!("{:?}", add(None));
} */

/*
#[derive(Debug)]
enum State {
    Alaska,
    Georgia,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("this quarter comes from {:?}.", state);
    } else {
        count += 1;
    }

    println!("{}", count);
} */

// enums

use std::{thread, time};

enum Event {
    Fetch(String),
    Pause { miliseconds: u64, message: String },
    Quit,
}

impl Event {
    fn inspect(self) {
        match self {
            Self::Fetch(thing) => println!("fetching {}...", thing),
            Self::Pause { miliseconds, message } => {
                println!("pausing for {} miliseconds: {}", miliseconds, message);
                thread::sleep(time::Duration::from_millis(miliseconds));
            }
            Self::Quit => println!("stopping all services."),
        }
    }
}

fn main() {
    let fetch: Event = Event::Fetch(String::from("https://crates.io/"));
    let pause: Event = Event::Pause { miliseconds: 1000, message: String::from("letting things cool...") };
    let quit:  Event = Event::Quit;

    fetch.inspect();
    pause.inspect();
    quit.inspect();
}

// match

/*
use rand::Rng;

fn evaluate_roll(roll: u8) -> &'static str {
    match roll {
        6 => "and you go!",
        1 => "pitiful.",
        _ => "bleh.",
    }
}

fn main() {
    let roll: u8 = rand::thread_rng().gen_range(1..=6);
    println!("you rolled {}, {}", roll, evaluate_roll(roll));
} */

// option

/*
fn call(thing: Option<u8>) {
    match thing {
        Some(t) => println!("value of something:\t{}", t),
        None => println!("value of nothing:\tnone"),
    }
}

fn main() {
    let k: u8 = 10;
    let some: Option<u8> = Some(20);
    let none: Option<u8> = None;

    call(some);
    call(none);

    assert_eq!(some.unwrap_or_else(|| 2 * k), 20);
    assert_eq!(none.unwrap_or_else(|| 2 * k), 20);
} */

// if let

/*
fn analyse(mood: &Option<String>) {
    if let Some(thing) = mood {
        println!("i am feeling {}!", thing);
    } else {
        println!("i dunno what i'm feeling =(.");
    }
}
fn main() {
    let happy: Option<String> = Some(String::from("happy"));
    let dunno: Option<String> = None;

    analyse(&happy);
    analyse(&dunno);
} */