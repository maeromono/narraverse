use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::Modifier, widgets::List};

use crate::{nvcore::narraverse::{Narraverse, NarraverseState}, utils::decosutils::{block_with_title, center_area}};

pub const PICKMENU_BINDINGS: &[(&str, &str)] = &[
    ("j/Down", "Navigate down"),
    ("k/Up", "Navigate up"),
    ("g", "Jump to top"),
    ("G", "Jump to bottom"),
    ("J", "Move selected entry down (entities, artifacts)"),
    ("K", "Move selected entry up (entities, artifacts)"),
    ("n", "Create a new entry"),
    ("e", "Edit selected entry"),
    ("Enter", "Enter editor"),
];

pub const EDITOR_BINDINGS: &[(&str, &str)] = &[
    ("Ctrl+PgDn", "Next buffer"),
    ("Ctrl+PgUp", "Previous buffer"),
    ("Alt+w", "Save buffer"),
    ("Alt+x", "Close buffer"),
    ("Ctrl+u", "Undo"),
    ("Ctrl+r", "Redo"),
    ("Alt+f", "Search in current buffer (regex supported)"),
    ("Shift+(Ctrl+)arrow keys", "Select text"),
    ("Ctrl+c", "Copy selected text"),
    ("Ctrl+x", "Cut selected text"),
    ("Ctrl+y", "Paste text (not from system clipboard)"),
    ("Ctrl+w", "Delete word before cursor"),
    ("Ctrl+Del", "Delete word AFTER cursor"),
];

#[derive(Clone, Copy)]
pub enum BindhelpMode {
    Pickmenu,
    Editor,
}

pub fn render_bindhelp(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let list = List::new(nv.cache.bindhelp_bindings_cache.iter().map(String::as_str))
        .highlight_style(Modifier::REVERSED)
        .block(block_with_title("Key bindings:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.bind_help_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => nv.list_states.bind_help_liststate.select_next(),
        KeyCode::Char('k') | KeyCode::Up => nv.list_states.bind_help_liststate.select_previous(),
        KeyCode::F(1) | KeyCode::Esc => {
            nv.list_states.bind_help_liststate.select(Some(0));
            nv.state = match nv.bindhelp_mode {
                BindhelpMode::Pickmenu => NarraverseState::VerseAreaPick,
                BindhelpMode::Editor => NarraverseState::Editor,
            }
        }
        _ => {}
    }
}
