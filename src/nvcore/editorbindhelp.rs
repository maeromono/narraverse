use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::Modifier, text::Line, widgets::List};

use crate::nvcore::{
    decosutils::{block_with_title, center_area},
    narraverse::{Narraverse, NarraverseState},
};

const EDITOR_BINDINGS: &[(&str, &str)] = &[
    ("Ctrl+PgDn", "Next buffer"),
    ("Ctrl+PgUp", "Previous buffer"),
    ("Alt+w", "Save buffer"),
    ("Alt+x", "Close buffer"),
    ("Ctrl+u", "Undo"),
    ("Ctrl+r", "Redo"),
    ("Shift+(Ctrl+)arrow keys", "Select text"),
    ("Ctrl+y", "Copy selected text"),
    ("Ctrl+x", "Cut selected text"),
    ("Ctrl+v", "Paste text (not from system clipboard)"),
    ("Ctrl+w", "Delete word before cursor"),
    ("Ctrl+Del", "Delete word AFTER cursor"),
];

pub fn render_bindhelp(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let lines: Vec<Line> = EDITOR_BINDINGS
        .iter()
        .map(|(bind, ac)| Line::from(format!("{}: {}", bind, ac)))
        .collect();
    let list = List::new(lines)
        .highlight_style(Modifier::REVERSED)
        .block(block_with_title("Editor key bindings:"));
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
        KeyCode::F(1) | KeyCode::Esc => nv.state = NarraverseState::Editor,
        _ => {}
    }
}
