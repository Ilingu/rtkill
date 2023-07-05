use tui::widgets::ListItem;

use crate::app::TargetDir;

use super::Renderer;

#[derive(Default)]
pub struct ListWithState<T> {
    pub index: usize,
    pub datas: Vec<T>,
}

impl<T> ListWithState<T> {
    pub fn next(&mut self) {
        match self.index >= self.datas.len() - 1 {
            true => self.index = 0,
            false => self.index += 1,
        }
    }
    pub fn previous(&mut self) {
        match self.index <= 1 {
            true => self.index = self.datas.len() - 1,
            false => self.index -= 1,
        }
    }
}

impl Renderer<Vec<ListItem<'static>>> for ListWithState<TargetDir> {
    fn render(&self) -> Vec<ListItem<'static>> {
        vec![]
    }
}
