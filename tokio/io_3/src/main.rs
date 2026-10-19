use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut f = File::create("boo.txt").await?;
    let n = f.write(b"hellohellohellohello").await?;
    println!("first {n} bytes were written");
    Ok(())
}
