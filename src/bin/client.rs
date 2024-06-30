use std::error::Error;

use cli_app::client::Connection;
use cli_app::logger;
use tracing::{info,error};
use std::io;
use std::io::Write;
use regex::Regex;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    info!("Connecting to server...");
    let addr = "127.0.0.1:8080";
    let mut client = Connection::new(addr).await?;
    let re = Regex::new(r#"request="([^"]+)""#).unwrap();

    
    info!("Please enter a command or type 'exit' to quit: ");
    loop{    
        print!("{addr}>");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Error getting guess");

        let input = input.trim();

        if input == "exit" {
            println!("Exiting...");
            return Ok(());
        }

        match client.run(input).await {
            Ok(response) => {
                if let Some(caps) = re.captures(&response) {
                    if let Some(matched) = caps.get(1) {
                        println!("{}", matched.as_str());
                    }
                } else {
                    println!("{}", response);
                }
            },
            Err(err) => error!("Error processing command: {}", err),
        }

        if let Err(err) = client.run(input).await {
            eprintln!("Error processing command: {}", err);
        }


}

}
