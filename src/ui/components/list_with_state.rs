use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{app::TargetDir, utils::FromHex};

use super::Renderer;

#[derive(Default)]
pub struct ListWithState<T> {
    pub index: usize,
    pub datas: Vec<T>,
}

impl<T> ListWithState<T> {
    pub fn current(&self) -> &T {
        &self.datas[self.index]
    }
    pub fn next(&mut self) {
        match self.index >= self.datas.len() - 1 {
            true => self.index = 0,
            false => self.index += 1,
        }
    }
    pub fn previous(&mut self) {
        match self.index == 0 {
            true => self.index = self.datas.len() - 1,
            false => self.index -= 1,
        }
    }
}

impl Renderer<()> for ListWithState<TargetDir> {
    /// takes a screen chunk and draw in it the target items components
    fn render_and_draw_items<B: Backend>(&self, f: &mut Frame<B>, chunks: Vec<Rect>) {
        if self.datas.is_empty() {
            return;
        }

        // all items cannot be displayed on screen thus this wil choose which items to display based on where is the currently selected one, it works by 'room': it search in which interval of "n" items the currently selected one is and display this interval
        let items_range = {
            let (mut inf, mut sup) = (0, chunks.len());
            while !(inf <= self.index + 1 && self.index < sup) {
                inf = sup;
                sup += chunks.len()
            }

            if sup >= self.datas.len() {
                sup = self.datas.len()
            }

            inf..sup
        };

        // for each items, render it's component
        let items = self.datas.iter().enumerate().collect::<Vec<_>>()[items_range].to_vec();
        for (slot_id, area) in chunks.iter().enumerate() {
            if slot_id >= items.len() {
                break;
            }
            let (item_id, item_data) = &items[slot_id];

            let item_block = if item_id == &self.index {
                Block::default().style(Style::default().fg(Color::Black).bg(Color::White))
            } else {
                Block::default()
            };
            f.render_widget(item_block, *area);

            let sub_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20), // 0: name
                    Constraint::Percentage(1),  // separator
                    Constraint::Percentage(65), // 2: path
                    Constraint::Percentage(1),  // separator
                    Constraint::Percentage(7),  // 4: last modified
                    Constraint::Percentage(1),  // separator
                    Constraint::Percentage(5),  // 6: size
                ])
                .split(*area);

            // "[DELETED]" if user has deleted this target folder otherwise the project name
            f.render_widget(
                match item_data.is_deleted {
                    true => Paragraph::new(Span::styled(
                        "[DELETED]",
                        Style::default()
                            .fg(Color::from_hex("#e74c3c").unwrap())
                            .add_modifier(Modifier::BOLD),
                    )),
                    false => Paragraph::new(item_data.project_name.clone()),
                },
                sub_chunks[0],
            );
            f.render_widget(Paragraph::new(item_data.path.clone()), sub_chunks[2]); // target path
            f.render_widget(
                Paragraph::new(item_data.last_modified.clone()), // last modified
                sub_chunks[4],
            );
            // target size
            f.render_widget(Paragraph::new(item_data.size.clone()), sub_chunks[6]);
        }
    }
}
