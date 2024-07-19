use crate::client;
use crate::utils::{Canvas, Chat, Message};
use colored::Colorize;
use irc::client::Client;

use irc::{
    client::{ClientBuilder, Motd},
    context::{ConnectionStatus, Context},
    event::Event,
    event_handler::EventHandler,
};
use std::error::Error;
use std::io::Stdout;
use std::sync::Arc;

pub struct Handler;

pub struct OxiChat {
    pub canvas: Canvas,
    pub client: Client,
}

impl OxiChat {
    pub async fn initialize_oxichat(
        arguments: Vec<String>,
    ) -> Result<(Self, std::io::Stdout), Box<dyn std::error::Error>> {
        let client = client::parsearg(arguments).await?;

        let (stdout, canvas) = Canvas::initialize_canvas()?;

        Ok((Self { client, canvas }, stdout))
    }
    pub async fn run_oxichat(mut self, stdout: Stdout) -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = self.client.connect().await {
            Err(Box::new(e))
            // panic!("Could not connect to server: {}", e);
        } else {
            println!("else block!");
            Ok(())
        }
    }
}

impl EventHandler for Handler {
    fn on_event(&self, ctx: Arc<Context>, event: Event) {
        match event {
            Event::RawMessage(_message) => {}

            Event::StatusChange => match *ctx.status {
                ConnectionStatus::Connecting => {
                    println!("{}", "Connecting to server".green().italic())
                }
                ConnectionStatus::Connected => {
                    println!("{}", "Connected to server".green().italic())
                }
                ConnectionStatus::Disconnected => {
                    println!("{}", "Disconnected from server".green().italic())
                }
            },
            Event::Notice(message) => {
                println!("{}: {}", "NOTICE".blue().bold(), message.as_str().yellow());
            }
            Event::ErrorMsg(message) => {
                println!("{}: {}", "ERROR".red().bold(), message.as_str().yellow());
            }
            Event::WelcomeMsg(message) => {
                println!("{}: {}", "WELCOME".blue().bold(), message.as_str().yellow());
            }

            Event::Motd => {
                if let Motd::Done(message) = ctx.motd.as_ref() {
                    println!("{}:\n{}", "MOTD".blue().bold(), message.as_str().cyan());
                }
            }

            Event::UnhandledMessage(message) => {
                println!(
                    "{}: {}",
                    "UNHANDLED".red().bold(),
                    String::try_from(message).unwrap().as_str().yellow()
                );
            }
        }
    }
}
