pub mod components;
use std::sync::Arc;

use tui::{backend::Backend, Frame};

use crate::{app::AppState, utils::sharable_state::SharableState};

pub fn ui<B: Backend>(f: &mut Frame<B>, state: &Arc<SharableState<AppState>>) {}
