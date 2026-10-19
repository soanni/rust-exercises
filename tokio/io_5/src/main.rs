use tokio::io;

use tokio::fs::File;

/// Note that this uses the fact that byte arrays also implement AsyncRead
/// More precisely byte slices
/// Look below for examples

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut f = File::create("bar.txt").await?;
    //let mut buf: &[u8] = b"hello the wonderful world!";
    // let mut buf = b"hello the wonderful world!" as &[u8];
    let mut buf = &b"hello the wonderful world!"[..];
    let n = io::copy(&mut buf, &mut f).await?;
    println!("{n} bytes have been copied between a slice and a file");
    // buffer is empty
    println!("{:?}", buf);
    Ok(())
}
