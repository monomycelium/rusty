fn main() {
    println!("Hello, world!");
    another_function(5, 'a');
}

fn another_function(x: i32, y: char) {
    println!("this is another function, which got a parameter of {x}{y}.")
}
