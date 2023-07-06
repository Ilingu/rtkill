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

/// Action to take when the message is deleted
pub enum MessageAction {
    Quit,
}

/// Describe an app message
pub struct Message {
    msg_text: String,
    msg_type: MessageType,
    duration: Duration,
    creation_date: Instant,
    pub action_when_deleted: Option<MessageAction>,
}

impl Message {
    pub fn new(
        msg_text: &str,
        msg_type: MessageType,
        duration: Option<Duration>,
        action: Option<MessageAction>,
    ) -> Self {
        Self {
            msg_text: msg_text.to_string(),
            msg_type,
            duration: duration.unwrap_or(Duration::from_secs(5)),
            creation_date: Instant::now(),
            action_when_deleted: action,
        }
    }

    pub fn should_be_deleted(&self) -> bool {
        self.creation_date.elapsed() >= self.duration
    }
}

impl Renderer<Paragraph<'static>> for Message {
    fn render_items(&self) -> Option<Paragraph<'static>> {
        let color = match self.msg_type {
            MessageType::Success => Color::from_hex("#2ecc71").unwrap(),
            MessageType::Info => Color::from_hex("#3498db").unwrap(),
            MessageType::Warning => Color::from_hex("#f1c90f").unwrap(),
            MessageType::Error => Color::from_hex("#e74c3c").unwrap(),
        };
        Some(
            Paragraph::new(self.msg_text.clone())
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
        )
    }
}
