use std::{env, fs, time::Duration};

use anyhow::{anyhow, Result};

use crate::ui::components::message::{Message, MessageAction, MessageType};

use super::AppState;

impl AppState {
    pub fn new() -> Result<Self> {
        match Self::from_args() {
            Ok(app) => Ok(app),
            Err(why) => {
                let parse_warn = Message::new(
                    "The provided root directory isn't valid. Current directory has been loaded instead.",
                    MessageType::Warning,
                    None,
                    None,
                );
                let parse_failed = Message::new(
                    "The provided root directory isn't valid. App will automatically quit in 10s.",
                    MessageType::Error,
                    Some(Duration::from_secs(10)),
                    Some(MessageAction::Quit),
                );

                match Self::from_cd() {
                    Ok(mut app) => {
                        if &why.to_string() != "bad args" {
                            app.message = Some(parse_warn);
                        }
                        Ok(app)
                    }
                    Err(_) => Ok(Self {
                        root_dir: None,
                        total_size: "0B".to_string(),
                        message: Some(parse_failed),
                        ..Default::default()
                    }),
                }
            }
        }
    }

    pub fn from_cd() -> Result<Self> {
        let root_dir = env::current_dir()?
            .to_str()
            .map(|rd| rd.to_string())
            .ok_or(anyhow!("couldn't load current directory"))?;
        Ok(Self {
            root_dir: Some(root_dir),
            total_size: "0B".to_string(),
            ..Default::default()
        })
    }

    pub fn from_args() -> Result<Self> {
        let args = env::args().skip(1).collect::<Vec<_>>();
        if args.len() != 1 {
            return Err(anyhow!("bad args"));
        }

        let root_dir = args[0].clone();
        if !fs::metadata(&root_dir)?.is_dir() {
            return Err(anyhow!("couldn't load provided directory"));
        }

        Ok(Self {
            root_dir: Some(root_dir),
            total_size: "0B".to_string(),
            ..Default::default()
        })
    }
}
