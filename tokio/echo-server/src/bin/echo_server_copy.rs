use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6349").await?;
    loop {
        let (mut tcp_stream, socket) = listener.accept().await?;
        tokio::spawn(async move {
            println!("accepted connection from {:?}", socket);
            process(&mut tcp_stream).await;
        });
    }
}

async fn process(stream: &mut TcpStream) {
    let (mut r, mut w) = stream.split();
    if tokio::io::copy(&mut r, &mut w).await.is_err() {
        eprintln!("failed to copy");
    }
}