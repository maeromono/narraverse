use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    widgets::{List, Paragraph},
};

use crate::{
    editor::{entityedit::EntityEditModal, versedelete::DeletionTarget},
    nvcore::{
        decosutils::{block_with_title, center_area},
        narraverse::{Narraverse, NarraverseState},
    },
};

pub fn render_entity_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    if verse.entities.len() == 0 {
        frame.render_widget(
            Paragraph::new("Hit n to create a new one.")
                .block(block_with_title("This Verse has no entities.")),
            center_area(area),
        );
        return;
    }
    let mut entity_list: Vec<String> = vec![];
    for entity in verse.entities.values() {
        entity_list.push(format!("[{}] {}", entity.entity_type.short_name(), entity.name));
    }
    let list = List::new(entity_list)
        .highlight_style(Modifier::REVERSED)
        .block(block_with_title("Pick an entity:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.verse_entity_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => nv.state = NarraverseState::VerseAreaPick,
        KeyCode::Char('j') | KeyCode::Down => {
            nv.list_states.verse_entity_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => {
            nv.list_states.verse_entity_pick_liststate.select_previous()
        }
        KeyCode::Char('n') => {
            nv.entity_edit_modal = EntityEditModal::default();
            nv.state = NarraverseState::EntityModal;
        }
        KeyCode::Char('e') => {
            let Some(index) = nv.list_states.verse_entity_pick_liststate.selected() else {
                return;
            };
            let Some(modal) = EntityEditModal::edit(nv, index) else {
                return;
            };
            nv.entity_edit_modal = modal;
            nv.state = NarraverseState::EntityModal;
        }
        KeyCode::Char('d') => {
            let Some(_) = nv.list_states.verse_entity_pick_liststate.selected() else {
                return;
            };
            nv.deletion_target = DeletionTarget::Entity;
            nv.state = NarraverseState::VerseDelete;
        }
        _ => {}
    }
}
