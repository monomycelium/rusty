pub trait Summary {
    fn summarise_author(&self) -> String;

    fn summarise(&self) -> String {
        format!("read more from {}...", self.summarise_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarise());
}

fn return_summarisable() -> impl Summary {
    Tweet {
        username: String::from("ayd3n"),
        content: String::from("sunny, isn't it?"),
        reply: false,
        retweet: false,
    }
} 

/*
pub fn notify<T: Summary>(item: &T) {
    println!("breaking news! {}", item.summarise());
} // more verbose way of doing the same thing.

pub fn notify<T: Summary>(item1: &T, item2: &T) // but at least we can choose to use the same types for two parameters.

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Display + Clone + std::fmt::Debug,
{
    let x = u;
    0
} // it may get messy, so use where
*/

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarise(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}", self.author)
    }
} // use default trait instead of overriding as you'd do above

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let t1 = Tweet {
        username: String::from("ayd3n"),
        content: String::from("sunny, isn't it?"),
        reply: false,
        retweet: false,
    };

    println!("one new tweet by {}", t1.summarise());

    let article = NewsArticle {
        headline: String::from("The Wall Must Be Built."),
        location: String::from("Washington, DC, USA"),
        author: String::from("Fox News Team"),
        content: String::from("\"We're going to build a wall, and you're paying for it.\"",),
    };

    println!("new article available: {}", article.summarise());

    notify(&article);

    println!("one new tweet by {}", return_summarisable().summarise());
}

/*
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

/*
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
} */

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest: x, {}", self.x);
        } else {
            println!("largest: y, {}", self.y);
        }
    }
}

// implement wildcard

// impl<T: Display> ToString for T {
// }

fn main() {
    let x = Pair { x: 9.0, y: 42.0 };
    
    x.cmp_display();
} */
