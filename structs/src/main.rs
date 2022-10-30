/*
fn main() {    
    // let mut powfu = User {
    //     email: String::from("powfu@amaz.ing"),
    //     username: String::from("powfu"),
    //     active: true,
    //     sign_in_count: 777,
    // };

    // powfu.sign_in_count += 1;
    // println!("{}", powfu.sign_in_count);

    let e1 = String::from("ayden@amaz.ing");
    let u1 = String::from("ayden");

    let ayden = build_user(e1, u1);

    let jonaz = User {
        username: String::from("jonaz"),
        ..ayden
    };
    
    println!("hi, i'm {}, and i am {}!", {ayden.username}, {jonaz.username});
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} */

/*
struct Color(i32, i32, i32);

fn main() {
    let woah = Color(0, 1, 2);
    println!("{}", woah.2);
} */

/*
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
} */
