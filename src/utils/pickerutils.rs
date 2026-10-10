use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;

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
        _ => false
    }
}
