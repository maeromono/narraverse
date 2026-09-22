use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::apcore::apverse::{ApVerse, ApVerseState};

pub fn render_narrative_editor(av: &mut ApVerse, frame: &mut Frame, area: Rect) {

}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => av.state = ApVerseState::VerseAreaPick,
        _ => {}
    }
}
