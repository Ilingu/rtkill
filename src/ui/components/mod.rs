pub mod list_with_state;
pub mod logo;
pub mod message;
pub mod rainbow_text;

pub trait Renderer<T> {
    fn render(&self) -> T;
}
