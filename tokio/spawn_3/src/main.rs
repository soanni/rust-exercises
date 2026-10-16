#[tokio::main]
async fn main() {
    tokio::task::spawn(async {
        {
            let rc = std::rc::Rc::new("hello");
            println!("{}", rc);
        }
        tokio::task::yield_now().await;
    });
}
