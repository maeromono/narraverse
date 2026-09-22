use ratatui::{layout::{Constraint, Rect}, widgets::{Block, Padding}};

pub fn bordered_block(title: &str) -> Block<'_> {
    Block::bordered().title(title).padding(Padding::proportional(1))
}

pub fn center_area(area: Rect) -> Rect {
    area.centered(Constraint::Max(75), Constraint::Max(15))
}
