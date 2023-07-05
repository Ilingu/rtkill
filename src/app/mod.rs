mod core;
mod parse;

use std::{sync::Arc, thread};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{
    ui::{components::list_with_state::ListWithState, ui},
    utils::sharable_state::SharableState,
};

#[derive(Default, Debug)]
pub struct TargetDir {
    pub path: String,
    pub project_name: String,
    pub last_modified: String,
}

#[derive(Default)]
pub struct AppState {
    pub root_dir: String,
    pub target_directories: ListWithState<TargetDir>,
    pub searching: bool,
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<SharableState<AppState>>,
) -> Result<()> {
    // search in parallel for target folders, and stream the data as it comes
    {
        let state_search = Arc::clone(&state);
        thread::spawn(move || state_search.search());
    }

    loop {
        println!("{:?}", state.read().target_directories.datas.len());
        terminal.draw(|f| ui(f, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up => (),
                KeyCode::Down => (),
                KeyCode::Left => (),
                KeyCode::Right => (),
                _ => (),
            };
        }
    }
}
