use traits_2::{NewsArticle, SocialPost, Summary};

fn main() {
    let article = NewsArticle {
        headline: "some awesome headline".to_string(),
        location: "California".to_string(),
        author: "soanni".to_string(),
        content: "some super awesome content".to_string(),
    };

    let post = SocialPost {
        username: "soanni".to_string(),
        content: "some awesome content".to_string(),
        repost: false,
        reply: true,
    };

    println!("some news ... {}", article.summarize());
    println!("new sozial post is here . {}", post.summarize());
}
