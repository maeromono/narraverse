use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, List},
};

pub fn block_with_title(title: &str) -> Block<'_> {
    Block::default().title(title).title_style(Modifier::BOLD)
}

pub fn center_area(area: Rect) -> Rect {
    area.centered(Constraint::Max(75), Constraint::Max(25))
}

pub fn focus_highlight(focused: bool) -> Style {
    if focused {
        Style::default().bold().reversed()
    } else {
        Style::default()
    }
}

// can't put the layout construction here bcuz non constant in a supposed constant field
pub fn fields_responsive(width: u16) -> (Direction, Constraint) {
    if width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    }
}

pub fn field_layout(direction: Direction) -> Layout {
    Layout::default()
        .direction(direction)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)])
}

pub fn list_highlight(list: List) -> List {
    list.highlight_style(Style::default().bold().reversed())
}
