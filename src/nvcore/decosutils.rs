use ratatui::{
    layout::{Constraint, Rect},
    widgets::Block,
};

pub fn bordered_block(title: &str) -> Block<'_> {
    Block::bordered().title(title)
}

pub fn center_area(area: Rect) -> Rect {
    area.centered(Constraint::Max(75), Constraint::Max(15))
}
