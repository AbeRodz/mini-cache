mod cmd;
mod cache;

use clap::Parser;
use cmd::Cli;
use cache::CacheDB;



fn main() {
    let cli = Cli::parse();
    let cache = CacheDB::new();
    
    match cli.command {
        cmd::Commands::Set { key, value } => {
            cache.set(key, value);
            println!("Set key-value pair.");
        }
        cmd::Commands::Get { key } => {
            match cache.get(key) {
                Some(value) => println!("Value: {}", value),
                None => println!("Key not found"),
            }
        }
        cmd::Commands::Del { key } => {
            cache.del(key);
            println!("Deleted key.");
        }
        cmd::Commands::List => {
            let keys = cache.list();
            println!("Keys: {:?}", keys);
        }
    }
}