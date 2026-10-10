use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::List,
};
use uuid::Uuid;

use crate::{
    editor::{configeditmodal::ConfigEditModal, delete::DeletionTarget, verseedit::VerseEdit},
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        verses::NarraverseVerses,
    }, utils::{cacheutils::{rebuild_art_list_cache, rebuild_ent_list_cache, rebuild_narr_list_cache}, decosutils::{block_with_title, center_area, list_highlight}, pickerutils::{render_list_empty_state, handle_list_keys}},
};

pub fn render_worldpick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    if nv.verses.verses.len() == 0 {
        render_list_empty_state("verses", frame, area);
        return;
    }
    let list = list_highlight(List::new(
        nv.cache.verse_list_cache.iter().map(String::as_str),
    ))
    .block(block_with_title("Pick a Verse:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.verse_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    if handle_list_keys(&mut nv.list_states.verse_pick_liststate, key) {
        return;
    }
    match key.code {
        KeyCode::Char('q') => nv.state = NarraverseState::Quit,
        KeyCode::Enter => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = selected_verse_uuid(&nv.verses, selected_index) else {
                return;
            };
            nv.verses.verse_uuid = uuid;
            nv.state = NarraverseState::VerseAreaPick;
            rebuild_narr_list_cache(&mut nv.cache, &nv.verses);
            rebuild_ent_list_cache(&mut nv.cache, &nv.verses);
            rebuild_art_list_cache(&mut nv.cache, &nv.verses);
        }
        KeyCode::Char('e') => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = selected_verse_uuid(&nv.verses, selected_index) else {
                return;
            };
            let Some(verse_edit) = VerseEdit::edit(&nv.verses, uuid) else {
                return;
            };
            nv.modals.verse_edit_modal = verse_edit;
            nv.state = NarraverseState::VerseEdit;
        }
        KeyCode::Char('n') => {
            nv.modals.verse_edit_modal = VerseEdit::default();
            nv.state = NarraverseState::VerseEdit;
        }
        KeyCode::Char('d') => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = selected_verse_uuid(&nv.verses, selected_index) else {
                return;
            };
            nv.verses.verse_uuid = uuid;
            nv.deletion_target = DeletionTarget::Verse;
            nv.state = NarraverseState::VerseDelete;
        }
        KeyCode::Char('c') => {
            nv.modals.config_edit_modal = ConfigEditModal::edit(&nv.config);
            nv.state = NarraverseState::ConfigModal;
        }
        _ => {}
    }
}

fn selected_verse_uuid(verses: &NarraverseVerses, ind: usize) -> Option<Uuid> {
    let verse = verses.verses.get_index(ind)?;
    Some(*verse.0)
}
