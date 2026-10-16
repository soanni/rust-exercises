/// When you spawn a task on the Tokio runtime, its type's lifetime must be 'static. This means that the spawned task must not contain any references to data owned outside the task.
#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // Now, the task owns all of its data, making it 'static.
    tokio::task::spawn(async move {
        println!("{:?}", v);
    });
}
