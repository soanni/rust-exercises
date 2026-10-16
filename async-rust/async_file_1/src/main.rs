use std::path::PathBuf;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncReadExt;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel(false);
    tokio::spawn(watch_file_changes(tx));

    loop {
        let _ = rx.changed().await;

        if let Ok(contents) = read_file("data.txt").await {
            println!("{contents}");
        }
    }
}

async fn read_file(f: &str) -> Result<String, std::io::Error> {
    let mut file = AsyncFile::open(f).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

async fn watch_file_changes(tx: watch::Sender<bool>) {
    let file_path = PathBuf::from("data.txt");

    let mut last_modified = None;

    loop {
        if let Ok(metadata) = file_path.metadata() {
            let modified = metadata.modified().unwrap();

            if Some(modified) != last_modified {
                last_modified = Some(modified);
                let _ = tx.send(true);
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}
