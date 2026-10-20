use bytes::BytesMut;
use mini_redis::{Frame, Result};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        // buffer 4k
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Read a frame from the Connection
    ///
    /// Returns 'None' if EOF is reached
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(f) = self.parse_frame()? {
                return Ok(Some(f));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    /// Write a frame to the connection
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        unimplemented!();
    }

    pub fn parse_frame(&mut self) -> Result<Option<Frame>> {
        unimplemented!();
    }
}

fn main() {}
