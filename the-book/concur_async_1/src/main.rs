use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 0..10 {
                println!("hi {i} from the 1st task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 1..5 {
            println!("hi {i} from the 2nd task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
