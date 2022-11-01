/*
fn main() {
    // let v: Vec<i32> = Vec::new();

    // let v = vec![1, 2, 3];

    let mut v = vec![5, 2, 9];

    v.push(3);
    v.push(3);
    v.push(6);

    // println!("index 2 of vector v is {}.", &v[2]);

    // let third: &i32 = &v[2];
    // println!("index 2 of vector v is {}.", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("third element: {}", third),
        None => println!("no third element!"),
    }

    for i in &v {
            print!("{}", i);
    }

    println!();

    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SheetCell::Int(999),
        SheetCell::Float(3.14),
        SheetCell::Text(String::from("hayes")),
    ];
} */

/*
fn main() {
    // let s1 = "lorem".to_string();
    let s1 = String::from("hello, ");
    let s2 = String::from("জোনাস");

    // let mut s3 = String::new();
    // s3.push_str(&s1);
    // s3.push_str(&s2);

    let mut s3 = s1 + &s2;
    s3.push('!');

    println!("{}", s3);

    // format

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = format!("{}-{}-{}", t1, t2, t3);
    
    println!("{}", t);

    // slice

    // println!("{}", s2.len());

    // let slice = &s2[0..6];
    // println!("{}", slice);

    for x in s2.chars() {
        print!("{}, ", x);
    }

    println!();
} */

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // print_scores(&scores);

    // let team = String::from("Blue");
    // let score = scores.get(&team).copied().unwrap_or(0);    
    // println!("{}: {}", team, score);
    
    scores.insert(String::from("Blue"), 60);
    // print_scores(&scores);

    scores.entry(String::from("Green")).or_insert(10);
    print_scores(&scores);

    word_database("lorem ipsum dolor sit amet");
}

fn print_scores(map: &HashMap<String, i32>) {
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}

fn word_database(text: &str) {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
