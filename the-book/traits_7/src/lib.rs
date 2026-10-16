use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub location: String,
    pub headline: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub repost: bool,
    pub reply: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}
// trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Smth new here ... {}", item.summarize());
}

// multiple trait bounds
// with where clause
pub fn notify_1<T>(item1: &T, item2: &T)
where
    T: Summary + Display,
{
    println!("Smth new here ... {}", item1.summarize());
    println!("Smth new here ... {}", item2.summarize());
}

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: "soanni1986".to_string(),
        content: "smth awesome-awesome".to_string(),
        repost: false,
        reply: false,
    }
}
