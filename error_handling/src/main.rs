use std::fs::File;
use std::io::ErrorKind;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = "../readme.md";

    let file_result = File::open(file);

    let output = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create(file).unwrap_or_else(|error| { panic!("cannot create file: {:?}.", error) }),
            ErrorKind::PermissionDenied => panic!("cannot access file: try executing `chown u+r {}` first.", file),
            other_error => panic!("cannot access file: {:?}.", other_error),
        },
    };

    print_file(output);
}

fn print_file(file: File) {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    
    let read_result = buf_reader.read_to_string(&mut contents);

    if let Err(error) = read_result {
        panic!("cannot read file: {:?}.", error);
    }
    
    println!("{}", contents);
}
