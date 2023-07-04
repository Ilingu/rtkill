mod core;
mod parse;

use std::thread;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::ui::{components::list_with_state::ListWithState, ui};

#[derive(Default, Debug)]
pub struct TargetDir {
    pub path: String,
    pub project_name: String,
    pub last_modified: String,
}

#[derive(Default)]
pub struct RTKill {
    pub root_dir: String,
    pub target_directories: ListWithState<TargetDir>,
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut RTKill) -> Result<()> {
    let _ = state.search(); // must run in parralel: Arc
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => (),
            };
        }
    }
}
