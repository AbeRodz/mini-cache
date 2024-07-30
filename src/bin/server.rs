use std::error::Error;
use std::time::Duration;
use tokio::net::TcpListener;
use cli_app::cache::CacheDB;
use cli_app::server::Listener;
use cli_app::logger;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    let cache = CacheDB::new(Duration::new(1, 0));
    let port = "127.0.0.1:8080";
    let listener = TcpListener::bind(port).await?;
    presentation();
    info!("Server running at {}", port);

    let mut server = Listener{
        db_conn: cache,
        listener: listener,
    };
    server.run().await?;
    Ok(())
}



fn presentation(){
    let ascii_art =r#"
     __    __     __     __   __     __     ______     ______     ______     __  __     ______    
    /\ "-./  \   /\ \   /\ "-.\ \   /\ \   /\  ___\   /\  __ \   /\  ___\   /\ \_\ \   /\  ___\   
    \ \ \-./\ \  \ \ \  \ \ \-.  \  \ \ \  \ \ \____  \ \  __ \  \ \ \____  \ \  __ \  \ \  __\   
     \ \_\ \ \_\  \ \_\  \ \_\\"\_\  \ \_\  \ \_____\  \ \_\ \_\  \ \_____\  \ \_\ \_\  \ \_____\ 
      \/_/  \/_/   \/_/   \/_/ \/_/   \/_/   \/_____/   \/_/\/_/   \/_____/   \/_/\/_/   \/_____/ 

    might be rusty but it's fast 
    "#;
    println!("{}", ascii_art);
}
