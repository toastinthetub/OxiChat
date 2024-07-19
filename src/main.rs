mod client;
mod error;
mod oxichat;
mod utils;

use irc::{client::ClientBuilder, event_handler::EventHandler};
use oxichat::OxiChat;
use std::env;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();

    let (mut oxichat, mut stdout) = OxiChat::initialize_oxichat(arguments).await.unwrap();

    let task = tokio::task::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        crate::utils::Canvas::euthanize().unwrap();
        std::process::exit(0);
    });

    if let Err(e) = oxichat.run_oxichat(stdout).await {
        panic!("Could not connect to server: {}", e);
    }

    _ = task.await;
}
