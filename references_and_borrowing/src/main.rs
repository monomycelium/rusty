// fn main() {
//     let s1 = String::from("hello");
//     let l1 = length_of(&s1);
//     println!("the length of {s1} is {l1}.");
// }

// fn length_of(s: &String) -> usize {
//     s.len()
// }

fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let l1 = length_of(&s1);

    println!("the length of '{s1}' is {l1}.");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn length_of(s: &String) -> usize {
    s.len()
}
