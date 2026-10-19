use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buf = Vec::new();

    let n = f.read_to_end(&mut buf).await?;

    println!("bytes read are: {:?}", &buf[..n]);
    Ok(())
}
