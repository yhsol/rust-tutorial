pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub fn run() {
    // impl Summarizable for NewsArticle {
    //     fn summary(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }

    impl Summarizable for NewsArticle {
        fn author_summary(&self) -> String {
            format!("author: {}", self.author)
        }
    }

    impl Summarizable for Tweet {
        fn author_summary(&self) -> String {
            format!("@{}", self.username)
        }

        // fn summary(&self) -> String {
        //     format!("{}: {}", self.username, self.content)
        // }
    }

    let article = NewsArticle {
        headline: String::from("Penguils win the Stanley Cup Chanpionship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguis once again are the best hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you  probably already know, people"),
        reply: false,
        retweet: false,
    };

    pub fn notify<T: Summarizable>(item: T) {
        println!("Breaking news! {}", item.summary());
    }

    println!("1 new tweet: {}", tweet.summary());
}
