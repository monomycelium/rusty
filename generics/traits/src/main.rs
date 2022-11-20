pub trait Summary {
    fn summarise(&self) -> String {
        String::from("read more...")
    }
}

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

impl Summary for NewsArticle {} // use default trait instead of overriding as you'd do above

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
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
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarise());
}
