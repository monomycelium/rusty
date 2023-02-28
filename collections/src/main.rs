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

// hashmaps
/*
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
} */

// vectors
/*
use std::io::{Write, self};
fn main() {
    let mut v: Vec<i32>;
    v = vec![1, 2, 3];

    v.push(4);

    let third: Option<&i32> = v.get(3);
    if let Some(thing) = third {
        println!("fifth element of vector: {}", thing);
    } else {
        println!("fifth element of vector: none");
    }

    for i in &mut v {
        *i += 12;
        print!("{}, ", i);
        io::stdout().flush().unwrap();
    }

    println!("boom!");
} */

// hashmaps recap

/*
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("black"), 165);
    scores.insert(String::from("red"), 130);
    
    let mut score: i32 = scores.get("black").copied().unwrap_or(0);
    println!("black house got {score} points!");

    for (key, value) in &scores {
        println!("{key} house:\t{value}");
    }

    // overwrite
    scores.insert(String::from("black"), 170);
    score = scores.get("black").copied().unwrap_or(0);
    println!("black house got {score} points!");

    // inserting if the key is absent
    scores.entry(String::from("yellow")).or_insert(115);
    score = scores.get("yellow").copied().unwrap_or(0);
    println!("yellow house got {score} points; =(.");

    // using the old value
    let string: &str = "cheng yi kai lewis is so epic jayden teng is so epic";
    let mut map: HashMap<&str, u8> = HashMap::new();

    for word in string.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{key}:\t{value}");
    }
} */

// practise

use std::io::{self, Write};

fn main() {
    println!("mean, median and mode finder written in rust.");
    print!("enter an array of integers separated by a space: ");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line.");

    let mut vector: Vec<i32> = vec![];
    for word in input.split_whitespace() {
        vector.push(word.parse::<i32>().expect("did you enter integers?"));
    }

    vector.sort();
    let len: usize = vector.len();
    if len == 0 {
        println!("did you enter integers?");
        std::process::exit(1);
    } else {
        let mean: f32 = vector.iter().sum::<i32>() as f32 / len as f32;
        println!("mean:\t{}", mean);

        if len % 2 > 0 {
            println!("median:\t{}", vector[(len + 1) / 2 - 1]);
        } else {
            println!("median:\t{}", (vector[len / 2] + vector[len / 2 - 1]) as f32 / 2.0);
        }

        // let mut map: HashMap<i32, u8> = HashMap::new();
        // for i in vector {
        //     let count: &mut u8 = map.entry(i).or_insert(0);
        //     *count += 1;
        // }
        // let max_value = map.values().cloned().max().unwrap_or(0);
        // let mode: i32 = map.into_iter()
        //     .filter(|&(_, v)| v == max_value)
        //     .collect();
        // println!("mode:\t{}", mode);
    }
}
