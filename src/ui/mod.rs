pub mod components;
use tui::{backend::Backend, Frame};

use crate::app::RTKill;

pub fn ui<B: Backend>(f: &mut Frame<B>, state: &RTKill) {}
