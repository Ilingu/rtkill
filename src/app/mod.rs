mod core;
mod parse;

use std::{
    fs,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Result};
use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{
    ui::{
        components::{
            list_with_state::ListWithState,
            message::{Message, MessageAction},
        },
        ui,
    },
    utils::sharable_state::SharableState,
};

/// Desscribes a rust 'target' folder
#[derive(Default, Debug, Clone)]
pub struct TargetDir {
    /// where it's located in the user disk
    pub path: String,
    /// rust associated project name in the Cargo.toml
    pub project_name: String,
    pub last_modified: String,
    /// Is user deleted the target file
    pub is_deleted: bool,
    /// formatted size of the folder, e.g: "5 GiB", "28 KiB"...
    pub size: String,
}

impl TargetDir {
    /// deletes permanently the folder from the user disk
    pub fn delete(&mut self) -> Result<()> {
        if self.is_deleted {
            return Err(anyhow!("folder already deleted"));
        }

        fs::remove_dir_all(self.path.clone())?;
        self.is_deleted = true;
        Ok(())
    }
}

/// Application public variables, persist after frame rebuild
#[derive(Default)]
pub struct AppState {
    pub root_dir: Option<String>,
    pub target_directories: ListWithState<TargetDir>,
    pub searching: bool,
    pub message: Option<Message>,
    pub total_size: String,
}

/// launch app, and begin frame
pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: Arc<SharableState<AppState>>,
) -> Result<()> {
    // search in parallel for target folders, and stream the data as it comes
    {
        let state_search = Arc::clone(&state);
        thread::spawn(move || state_search.search());
    }

    let mut last_time_user_active = Instant::now();
    loop {
        let current_appstate = Arc::new(state.read()); // I wrap it in Arc to prevent calling the read unsafe function several times within a frame (loop iteration), like this data is updated once every frame

        // build and show ui
        let ui_state = Arc::clone(&current_appstate);
        terminal.draw(|f| ui(f, &ui_state))?;

        // check message deletion
        if let Some(msg) = &current_appstate.message {
            if msg.should_be_deleted() {
                if let Some(MessageAction::Quit) = &msg.action_when_deleted {
                    return Ok(());
                }

                state.set_message(None);
            }
        }

        // check events
        let refresh_rate = match last_time_user_active.elapsed().as_secs() >= 10 {
            true => 1000,
            false => 100,
        }; // if user active, refresh view every 1 second, otherwise 10 time per second
        if event::poll(Duration::from_millis(refresh_rate))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => state.prev_item(),
                    KeyCode::Down => state.next_item(),
                    KeyCode::Char(' ') => state.delete_current_item(),
                    KeyCode::Char('r') => {
                        // to avoid user to spam refresh, which could cause memory issue
                        if !current_appstate.searching {
                            state.clear_list();
                            {
                                let state_search = Arc::clone(&state);
                                thread::spawn(move || state_search.search());
                            }
                        }
                    }
                    _ => (),
                };
            }
            last_time_user_active = Instant::now();
        }
    }
}
