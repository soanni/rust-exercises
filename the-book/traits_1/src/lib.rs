pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub location: String,
    pub headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub repost: bool,
    pub reply: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}
