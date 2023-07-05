mod app;
mod ui;
mod utils;

use std::sync::Arc;

use anyhow::Result;
use app::{run_app, AppState};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};
use utils::sharable_state::SharableState;

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // initial state
    let state = match AppState::from_args() {
        Ok(d) => d,
        Err(_) => {
            AppState::from_cd().expect("Couldn't get your current directory, please provide it")
        }
    };
    let sharable_state = Arc::new(SharableState::new(state));

    // app launch
    let app_quit_result = run_app(&mut terminal, sharable_state);

    // app quit
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    if let Err(e) = app_quit_result {
        println!("{}", e);
    }
    Ok(())
}
