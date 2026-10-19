use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buf = [0; 10];

    let n = f.read(&mut buf).await?;

    println!("bytes read are: {:?}", &buf[..n]);
    Ok(())
}
