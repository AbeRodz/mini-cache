
use std::error::Error;
use bytes::BytesMut;
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;
use std::str;
pub struct Connection {
connection : TcpStream,
buffer : BytesMut
}

impl Connection{


pub async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
    let connection = TcpStream::connect(addr).await?;
    Ok(Self {
        connection,
        buffer: BytesMut::with_capacity(1024),
    })
}
    pub async fn run(&mut self, command : &str)-> Result<String, Box<dyn Error>>{
        self.write(command).await?;

        let response = self.response().await?;

        Ok(response)
    }

    async fn write(&mut self, command : &str) -> Result<(), Box<dyn Error>>{

        loop {
            self.connection.writable().await?;

            match self.connection.try_write(command.as_bytes()) {
                Ok(_) => {
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        Ok(())

    }
    async fn response(&mut self) -> Result<String, Box<dyn Error>> {
        let bytes_read = self.connection.read_buf(&mut self.buffer).await?;
        
        if bytes_read == 0 {
            if self.buffer.is_empty() {
                return Ok("empty buffer".to_string());
            } else {
                return Err("connection reset by peer".into());
            }
        }
        let mut response = String::new();
        while let Some(pos) = self.buffer.iter().position(|&b| b == b'\n') {

            let line = self.buffer.split_to(pos + 1);
            let line = &line[..line.len() - 1]; 
            
            if let Ok(request) = str::from_utf8(line) {
                response = request.to_string();

            }
        }

        Ok(response)
    }
}

