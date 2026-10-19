use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut f = File::create("boo.txt").await?;
    f.write_all(b"hellohellohellohello").await?;
    Ok(())
}
