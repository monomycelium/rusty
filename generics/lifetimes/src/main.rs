/*
fn main() {
    let s1: &str = "lorem";
    let s2: &str = "ipsum";

    let result = longest(s1, s2);
    println!("longest string: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

struct Impt<'a> {
    part: &'a str,
}

fn main() {
    let p1: String = String::from("we're building a wall. and you're paying for it.");
    let s1: &str = p1.split('.').next().expect("period?");

    let i1: Impt = Impt { part: s1 };

    println!("my point is, {}.", i1.part);
}
