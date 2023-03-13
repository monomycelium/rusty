use std::process::exit;

fn main() {
    println!("i love you, lewis!");

    let tasks: Vec<lewis::Task> = vec![
        lewis::Task {
            completed: false,
            description: String::from("hi there."),
        },
        lewis::Task {
            completed: false,
            description: String::from("cry about it."),
        },
    ];

    if let Err(or) = lewis::run(tasks) {
        eprintln!("application error: {}.", or);
        exit(1);
    };
}
