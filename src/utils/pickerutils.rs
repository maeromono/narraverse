use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{ListState, Paragraph},
};

use crate::utils::decosutils::{block_with_title, center_area};

pub fn render_list_empty_state(section: &str, frame: &mut Frame, area: Rect) {
    let title = format!("You have no {}.", section);
    frame.render_widget(
        Paragraph::new("Press n to create a new one.").block(block_with_title(&title)),
        center_area(area),
    );
}

pub fn handle_list_keys(list_state: &mut ListState, key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            list_state.select_next();
            true
        }
        KeyCode::Char('k') | KeyCode::Up => {
            list_state.select_previous();
            true
        }
        KeyCode::Char('g') | KeyCode::Home => {
            list_state.select_first();
            true
        }
        KeyCode::Char('G') | KeyCode::End => {
            list_state.select_last();
            true
        }
        _ => false,
    }
}
