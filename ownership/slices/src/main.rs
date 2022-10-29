// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[..5];
//     let world = &s[6..];

//     println!("{} {}", hello, world);
// }

fn main() {
    let word = n_word("hello world");

    println!("{}", word);
}

fn n_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i] // return from beginning to letter i of the string
        }
    }

    &s[..] // return whole string if there are no spaces
}
