use std::error::Error;
use std::time::Duration;
use crate::cache::CacheDB;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener,TcpStream}};
use tracing::{info,error};

#[derive(Debug)]
struct Handler{
    db : CacheDB,
    stream : TcpStream
}
impl Handler {

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>>{
        let mut buffer = [0;1024];
        loop {
            let n = match self.stream.read(&mut buffer).await  {
                Ok(n) if n == 0 => return Ok(()),
                Ok(n) => n,
                Err(err) => {
                    error!("{:?}failed reading from socket", err);
                    return Err(Box::new(err));
                }
            };
            let request = String::from_utf8_lossy(&buffer[..n]);
            let response = self.request_handler(&request).await;

            self.stream.write_all(response.as_bytes()).await?;
        }
    }
    async fn request_handler(&mut self, request: &str) -> String{
        let parts: Vec<&str> = request.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["SET", key, value, expiration_str ] => {
                let expiration_duration = match expiration_str.parse::<u64>() {
                    Ok(seconds) => Some(Duration::new(seconds, 0)),
                    Err(_) => return "ERROR: Invalid expiration time\n".to_string(),
                };
                self.db.set(key.to_string(), value.to_string(),expiration_duration);
                "OK\n".to_string()
            }
            ["SET", key, value] => {
                // No expiration provided
                self.db.set(key.to_string(), value.to_string(), None);
                "OK\n".to_string()
            }
            ["GET", key] => {

                match self.db.get(key.to_string()) {
                    Some(value) => format!("{}\n", value),
                    None => "ERROR: Key not found or expired\n".to_string(),
                }
            }
            ["DEL", key] => {

                self.db.del(key.to_string());
                "OK\n".to_string()
            }
            ["LIST"] => {

                let keys: Vec<String> = self.db.list();
                format!("{}\n", keys.join(" "))
            }
            ["TTL", key] => {

                match self.db.get_ttl(key.to_string()) {
                    Some(value) => format!("TTL: {} seconds\n", value.as_secs()),
                    None => "ERROR: Key not found or expired\n".to_string(),
                }
            }
            _ => "ERROR: Unknown command\n".to_string(),
        }

    }
}

#[derive(Debug)]
pub struct  Listener{
    pub db_conn : CacheDB,
    pub listener : TcpListener,
}

impl Listener {
   pub  async fn run(& mut self)-> Result<(), Box<dyn Error>>{
        info!("listening inbound connections...");
        loop {
            let socket = self.accept().await?;
            let db_conn = self.db_conn.clone();
            let mut handler = Handler{
                db: db_conn,
                stream: socket,
            };

        tokio::spawn(async move {
            if let Err(err) = handler.run().await {
                error!("error {} connecting to client", err);
            }
        });
        }
    }
    async fn accept(&mut self) -> Result<TcpStream, Box<dyn Error>>{
        match self.listener.accept().await{
            Ok((socket, _)) => return  Ok(socket),
            Err(err) => {
                error!("failed to accept connection: {:?}", err);
                Err(err.into())
            }
        }
        
    }
}
