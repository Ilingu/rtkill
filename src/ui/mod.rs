pub mod components;
mod info_section;
mod list_section;

use self::{
    components::rainbow_text::rainbow_text, info_section::draw_info_section,
    list_section::draw_list_section,
};
use crate::app::AppState;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

/// root function to build and draw on the screen all the app ui
pub fn ui<B: Backend>(f: &mut Frame<B>, state: &AppState) {
    let parent_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(f.size());

    let info_section = Block::default()
        .title(rainbow_text("RTKILL::Rust Target Killer"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(info_section, parent_chunk[0]);
    draw_info_section(f, parent_chunk[0], state);

    let list_section = Block::default()
        .title(rainbow_text("Target directories"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightRed));
    f.render_widget(list_section, parent_chunk[1]);
    draw_list_section(f, parent_chunk[1], state);
}
