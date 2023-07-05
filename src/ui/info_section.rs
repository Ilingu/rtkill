use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame,
};

use crate::app::AppState;

use super::components::{logo::welcome_logo, Renderer};

pub fn draw_info_section<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);
    f.render_widget(welcome_logo().alignment(Alignment::Center), chunks[0]);

    let sub_chunck = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    if let Some(message) = &state.message {
        f.render_widget(message.render().alignment(Alignment::Center), sub_chunck[0]);
    } else if state.searching {
        f.render_widget(
            Paragraph::new(format!(
                "Searching 'target' directories ⏳ Search scope: {}",
                state.root_dir
            ))
            .alignment(Alignment::Center),
            sub_chunck[0],
        );
    } else {
        f.render_widget(
            Paragraph::new(Spans::from(vec![
                Span::raw("Found "),
                Span::styled(
                    state.target_directories.datas.len().to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" 'target' directories"),
            ]))
            .alignment(Alignment::Center),
            sub_chunck[0],
        );
    }

    f.render_widget(
        Paragraph::new(Spans::from(vec![
            Span::styled(
                "Movement: ",
                Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::styled(
                "Up/Down",
                Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
            Span::raw(" // "),
            Span::styled("Action: ", Style::default().add_modifier(Modifier::ITALIC)),
            Span::styled(
                "Space (Delete)",
                Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
        ]))
        .alignment(Alignment::Center),
        sub_chunck[1],
    );
}
