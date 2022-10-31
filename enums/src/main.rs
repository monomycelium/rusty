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
