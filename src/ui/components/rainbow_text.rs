use lazy_static::lazy_static;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

lazy_static! {
    static ref RAINBOW_COLORS: [Color; 9] = [
        Color::Rgb(255, 173, 173),
        Color::Rgb(255, 214, 165),
        Color::Rgb(253, 255, 182),
        Color::Rgb(202, 255, 191),
        Color::Rgb(155, 246, 255),
        Color::Rgb(160, 196, 255),
        Color::Rgb(189, 178, 255),
        Color::Rgb(255, 198, 255),
        Color::Rgb(255, 255, 252),
    ];
}

/// create a raibow text
///
/// it return a collection of Span (Spans), to display it use can use a widget like `Paragraph`
///
/// e.g:
///
/// ```
/// Paragraph::new(rainbow_text("rainbow!"));
/// ```
pub fn rainbow_text(text: &str) -> Spans<'static> {
    let mut colored_text: Vec<Span> = vec![];
    for (i, ch) in text.chars().enumerate() {
        colored_text.push(Span::styled(
            ch.to_string(),
            Style::default().fg(RAINBOW_COLORS[i % RAINBOW_COLORS.len()]),
        ));
    }

    Spans::from(colored_text)
}
