/*
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;

fn main() {
    let f1 = "../readme.md";

    // let file_result = File::open(f1);

    // let o1 = match file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => File::create(f1).unwrap_or_else(|error| { panic!("cannot create file: {:?}.", error) }),
    //         ErrorKind::PermissionDenied => panic!("cannot access file: try executing `chown u+r {}` first.", f1),
    //         other_error => panic!("cannot access file: {:?}.", other_error),
    //     },
    // };

    // print_file(o1);

    // let o2 = File::open("../../readme.md").unwrap();

    let o1 = File::open(f1).expect("does {f1} exsist?");

    print_file(o1);
}

fn print_file(file: File) {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    let read_result = buf_reader.read_to_string(&mut contents);

    if let Err(error) = read_result {
        panic!("cannot read file: {:?}.", error);
    }

    println!("{}", contents);
} */

use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let file_path = "../readme.md";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let c1 = read_file();

    match c1 {
        Ok(c) => println!("{}", c),
        Err(e) => panic!("can't access file: {:?}", e),
    }
}
