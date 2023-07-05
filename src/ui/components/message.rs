use std::time::{Duration, Instant};

use tui::{
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use crate::utils::FromHex;

use super::Renderer;

pub enum MessageType {
    Success,
    Info,
    Warning,
    Error,
}

pub struct Message {
    msg_text: String,
    msg_type: MessageType,
    duration: Duration,
    creation_date: Instant,
}

impl Message {
    pub fn new(msg_text: &str, msg_type: MessageType, duration: Option<Duration>) -> Self {
        Self {
            msg_text: msg_text.to_string(),
            msg_type,
            duration: duration.unwrap_or(Duration::from_secs(5)),
            creation_date: Instant::now(),
        }
    }

    pub fn should_be_deleted(&self) -> bool {
        self.creation_date.elapsed() >= self.duration
    }
}

impl Renderer<Paragraph<'static>> for Message {
    fn render(&self) -> Paragraph<'static> {
        let color = match self.msg_type {
            MessageType::Success => Color::from_hex("#2ecc71").unwrap(),
            MessageType::Info => Color::from_hex("#3498db").unwrap(),
            MessageType::Warning => Color::from_hex("#f1c40f").unwrap(),
            MessageType::Error => Color::from_hex("#e74c3c").unwrap(),
        };
        Paragraph::new(self.msg_text.clone())
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
    }
}
