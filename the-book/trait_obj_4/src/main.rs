use trait_obj_4::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("some text");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("some text", post.content());
}
