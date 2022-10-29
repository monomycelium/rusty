/*
fn main() {
    // let x = 5;
    // println!("value of x: {x}");
    // let x = x + 1;
    // println!("value of x: {x}"); // this allows the new variable x to be of ANY type!

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("amt. char.: {spaces}")

    // let tup = (529, 'j', 336);
    // let (x, y, z) = tup;
    // println!("first letter of my favourite person: {y}.")

    // let x: (i32, char, u8) = (500, 'j', 1);
    // let yay = x.1;
    // println!("yay, {yay}!")

    // let a = [5, 2, 9, 3, 3, 6];
    // println!("first element: {}", a[0]);

    // {
    //     let s = "hello";
    //     println!("s exists? {s}");
    // }

    // let mut s = String::from("hello");
    // s.push_str(" world");
    // println!("{s}!");

    // let x = 5;
    // let y = x;
    // let x = 10;
    // println!("({x}, {y})");

    // let x = String::from("hello");
    // let y = x;
    // println!("{y}, {y}");

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    let y = s.clone();
    own(s);

    let x = 5;
    copy(x);

    println!("{y}, {x}");
}

fn own(string: String) {
    println!("{}", string);
}

fn copy(integer: i32) {
    println!("{}", integer);
}*/

/*
fn main() {
    let s1 = String::from("hello");
    let s2 = borrow_and_return(s1);
    println!("{s2}");
}

fn borrow_and_return(string: String) -> String {
    string
}*/


fn main() {
    let s1 = String::from("hello");
    let (s2, length) = length_of(s1);
    println!("length of {s2} is {length}.");
}

fn length_of(string: String) -> (String, usize) {
    let length = string.len();
    (string, length)
}
