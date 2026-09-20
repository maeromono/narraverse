use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::Style,
    widgets::{Block, List, ListState},
};

use crate::apcore::apverse::{ApVerse, ApVerseState};

pub struct ApVerseMainMenu {
    sidebar_state: ListState,
}

impl Default for ApVerseMainMenu {
    fn default() -> Self {
        Self {
            sidebar_state: ListState::default().with_selected(Some(0)),
        }
    }
}

pub fn render_menu_sidebar(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let menu = vec!["New world...", "Settings", "Quit"];
    let menu_list = List::new(menu)
        .block(Block::bordered())
        .highlight_style(Style::default().reversed());
    frame.render_stateful_widget(menu_list, area, &mut av.main_menu.sidebar_state);
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => av.main_menu.sidebar_state.select_next(),
        KeyCode::Char('k') | KeyCode::Up => av.main_menu.sidebar_state.select_previous(),
        KeyCode::Char(' ') | KeyCode::Enter => {
            match av.main_menu.sidebar_state.selected().unwrap_or_default() {
                0 => av.state = ApVerseState::Editor,
                2 => av.state = ApVerseState::Quit,
                _ => {}
            }
        }
        _ => {}
    }
}
