use ratatui::{
    layout::{Constraint, Rect}, style::Modifier, widgets::Block
};

pub fn block(title: &str) -> Block<'_> {
    Block::default().title(title).title_style(Modifier::BOLD)
}

pub fn center_area(area: Rect) -> Rect {
    area.centered(Constraint::Max(75), Constraint::Max(15))
}
