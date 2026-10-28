use trait_obj_3::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("some text");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("some text", post.content());
}
