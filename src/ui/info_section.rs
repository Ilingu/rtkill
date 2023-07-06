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

/// draw the ui for the inner top section of the app
pub fn draw_info_section<B: Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    // divide the space in two chuncks of 80% (for the logo) and 20% (app infos)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);
    f.render_widget(welcome_logo().alignment(Alignment::Center), chunks[0]); // render logo

    // redivide the 20% for the app infos, in two equal sub-chunks for the message/app info and the controls
    let sub_chunck = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // if message renders it in priority, otherwise render searching result if result or otherwise the searching state
    if let Some(message) = &state.message {
        f.render_widget(
            message.render_items().unwrap().alignment(Alignment::Center),
            sub_chunck[0],
        );
    } else if state.searching {
        f.render_widget(
            Paragraph::new(format!(
                "Searching 'target' directories ⏳ Search scope: {}",
                state.root_dir.clone().unwrap_or("undefined".to_string())
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
                Span::raw(" // "),
                Span::raw("Total size: "),
                Span::styled(
                    state.total_size.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]))
            .alignment(Alignment::Center),
            sub_chunck[0],
        );
    }

    // render controls
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
            Span::raw(", "),
            Span::styled(
                "q (Quit)",
                Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
            Span::raw(", "),
            Span::styled(
                "r (Refresh)",
                Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
        ]))
        .alignment(Alignment::Center),
        sub_chunck[1],
    );
}
