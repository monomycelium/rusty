/*
fn main() {
    let list: Vec<i32> = vec![53, 34, 45, 85, 25];

    let largest: i32 = get_largest(list);

    println!("largest number: {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest: &T = &list[0];
    for number in &list {
        if number > largest {
            largest = number;
        }
    }
    
    largest
} */

/*
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // let integer = Point { x: 3, y: 5 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let mixed = Point { x: 4, y: 9.0 };

    let p1: Point<i32> = Point { x: 5, y: 4 };
    println!("p.x = {}", p1.x());

    let p2: Point<f32> = Point { x: 5.0, y: 4.0 };
    println!("distance from origin: {}", p2.distance_from_origin());
} */
/*
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mix<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }    
}

fn main() {
    let p1 = Point { x: 5, y: 'c' };
    let p2 = Point { x: "hello", y: 82.4 };

    let p3 = p1.mix(p2);

    println!("({}, {})", p3.x, p3.y);
} */

// using generics in functions

use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numeric_list: [i32; 6] = [5, 2, 9, 3, 3, 6];
    println!("largest number:\t\t{}", largest(&numeric_list));

    let numeric_char: [char; 6] = ['j', 'a', 'y', 'd', 'e', 'n'];
    println!("largest character:\t{}", largest(&numeric_char));
}
