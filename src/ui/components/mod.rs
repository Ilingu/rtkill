use tui::{backend::Backend, layout::Rect, Frame};

pub mod list_with_state;
pub mod logo;
pub mod message;
pub mod rainbow_text;

/// simple trait to origanize how component renders their ui
pub trait Renderer<T> {
    fn render_items(&self) -> Option<T> {
        None
    }
    fn render_and_draw_items<B: Backend>(&self, _: &mut Frame<B>, _: Vec<Rect>) {}
}
