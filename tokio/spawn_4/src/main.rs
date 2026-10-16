/// Tasks spawned by tokio::spawn must implement Send. This allows the Tokio runtime to move the tasks between threads while they are suspended at an .await.

/// Tasks are Send when all data that is held across .await calls is Send. This is a bit subtle. When .await is called, the task yields back to the scheduler. The next time the task is executed, it resumes from the point it last yielded. To make this work, all state that is used after .await must be saved by the task. If this state is Send, i.e. can be moved across threads, then the task itself can be moved across threads. Conversely, if the state is not Send, then neither is the task.

#[tokio::main]
async fn main() {
    tokio::task::spawn(async {
        let rc = std::rc::Rc::new("hello");
        println!("{}", rc);
        tokio::task::yield_now().await;
    });
}
