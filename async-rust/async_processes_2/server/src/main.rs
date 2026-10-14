use tokio::process::Command;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut handles = vec![];

    for _ in 0..4 {
        let handle = tokio::spawn(async {
            let out = Command::new("./connection_bin").output().await;

            match out {
                Ok(o) => {
                    println!(
                        "process completed with output {}",
                        String::from_utf8_lossy(&o.stdout)
                    );
                    Ok(o.status.code().unwrap_or(-1))
                }
                Err(e) => {
                    eprintln!("failed to run process: {}", e);
                    Err(e)
                }
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(handles.len());

    for h in handles {
        results.push(h.await.unwrap());
    }

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(exit_code) => println!("process {} exited with code {}", i + 1, exit_code),
            Err(e) => eprintln!("process {} failed {}", i + 1, e),
        }
    }

    Ok(())
}
