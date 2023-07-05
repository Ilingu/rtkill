use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::AppState;

use super::components::Renderer;

pub fn draw_list_section<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1)].repeat(33))
        .split(area);
    state.target_directories.render_and_draw_items(f, chunks)
}
