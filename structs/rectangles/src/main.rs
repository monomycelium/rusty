/*
fn main(){
    let w1 = 30;
    let h1 = 50;

    println!(
        "the area of the rectangle is {} square pixels.",
        area_of(w1, h1)
    );
}

fn area_of(w: u32, h: u32) -> u32 {
    w * h
} */

struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }

    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        w: 30,
        h: 50,
    };

    let r2 = Rectangle {
        w: 70,
        h: 40,
    };

    let s1 = Rectangle::square(30);

    println!(
        "the area of the rectangle is {} square pixels.",
        r1.area()
    );

    println!(
        "rectangle `r1` can hold rectangle `r2`: {}",
        r1.can_hold(&r2)
    );

    println!{
        "rectangle `r2` can hold square `s1`: {}",
        r2.can_hold(&s1)
    };
}
