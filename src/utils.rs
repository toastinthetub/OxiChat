use crossterm::{
    cursor::MoveTo,
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use irc::{
    event_handler::EventHandler,
    message::{self, IrcMessage},
};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Canvas {
    pub size: (u32, u32), // w, h
    pub chat: Chat,
}

pub struct Chat {
    pub messages: Vec<Message>,
    pub context: (u32, u32),
    pub newline_index: u16,
}

pub struct Message {
    pub timestamp: String,
    pub author: String,
    pub content: String,
    pub position: u32,
}

impl Canvas {
    pub fn initialize_canvas() -> Result<(std::io::Stdout, Self), Box<dyn std::error::Error>> {
        let mut stdout: std::io::Stdout = std::io::stdout();
        let (w, h) = crossterm::terminal::size()?;
        let _ = stdout.queue(crossterm::terminal::EnterAlternateScreen);
        crossterm::terminal::enable_raw_mode()?;
        stdout.queue(Clear(ClearType::All))?;
        std::io::Write::flush(&mut stdout)?;

        let chat = Chat::new();

        Ok((
            stdout,
            Self {
                size: (w.into(), h.into()),
                chat,
            },
        ))
    }
    pub fn euthanize() -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout: std::io::Stdout = std::io::stdout();
        crossterm::terminal::disable_raw_mode()?;
        let _ = stdout.queue(crossterm::terminal::LeaveAlternateScreen);
        std::io::Write::flush(&mut stdout).unwrap();
        Ok(())
    }
}

impl Chat {
    pub fn new() -> Self {
        let mut messages: Vec<Message> = Vec::new();
        for i in 0..=5 {
            let message = format!("this,is,message,{}", i);
            let message = Message::from_str(&message).unwrap();
            messages.push(message);
        }
        let context: (u32, u32) = (50, 50);
        let newline_index: u16 = 0;
        Self {
            messages,
            context,
            newline_index,
        }
    }
}

impl Message {
    pub fn from_irc_message(message: String) -> Self {}
}

impl FromStr for Message {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return Err("Input string does not have exactly 4 parts".to_string());
        }

        let position: u32 = parts[3]
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;

        Ok(Message {
            timestamp: parts[0].to_string(),
            author: parts[1].to_string(),
            content: parts[2].to_string(),
            position,
        })
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!(
            "time: {} {} {} {}",
            self.timestamp, self.author, self.content, self.position
        )
    }
}
