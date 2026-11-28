use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{ListState, Paragraph},
};

use crate::{
    nvcore::versemanifest::VerseManifest,
    utils::decosutils::{block_with_title, center_area},
};

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

pub fn verse_pick_items(manifest: &VerseManifest) -> Vec<String> {
    manifest
        .verses
        .values()
        .map(|verse| verse.name.clone())
        .collect()
}

pub fn narrative_pick_items(manifest: &VerseManifest) -> Vec<String> {
    manifest
        .current_verse()
        .narratives
        .values()
        .map(|narr| format!("[{}] {}", narr.date, narr.title))
        .collect()
}

pub fn entity_pick_items(manifest: &VerseManifest) -> Vec<String> {
    manifest
        .current_verse()
        .entities
        .values()
        .map(|ent| format!("[{}] {}", ent.entity_type.short_name(), ent.name))
        .collect()
}
