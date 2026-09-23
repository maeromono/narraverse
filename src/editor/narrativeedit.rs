use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use ratatui_textarea::TextArea;

use crate::apcore::{apverse::{ApVerse, ApVerseState}, verses::Narrative};

pub struct NarrativeEditor {
    pub currently_editing: usize,
    pub textarea: TextArea<'static>,
}

impl Default for NarrativeEditor {
    fn default() -> Self {
        Self { currently_editing: 0, textarea: TextArea::default() }
    }
}

impl NarrativeEditor {
    pub fn edit(narrative: &Narrative) -> Self {
        Self { currently_editing: 0, textarea: TextArea::from(narrative.narrative.lines()) }
    }
}

pub fn render_narrative_editor(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let editor_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => av.state = ApVerseState::VerseAreaPick,
        _ => {}
    }
}
