use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, widgets::List};
use uuid::Uuid;

use crate::{
    editor::{configeditmodal::ConfigEditModal, delete::DeletionTarget, verseedit::VerseEdit},
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        versemanifest::VerseManifest,
    },
    utils::{
        cacheutils::{rebuild_art_list_cache, rebuild_ent_list_cache, rebuild_narr_list_cache},
        decosutils::{block_with_title, center_area, list_highlight},
        pickerutils::{handle_list_keys, render_list_empty_state, verse_pick_items},
    },
};

pub fn render_worldpick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    if nv.get_verse_manifest().no_verses() {
        render_list_empty_state("verses", frame, area);
        return;
    }
    let list = list_highlight(List::new(verse_pick_items(nv.get_verse_manifest())))
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
        KeyCode::Char('q') => nv.set_state(NarraverseState::Quit),
        KeyCode::Enter => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = nv
                .get_verse_manifest()
                .current_verse_uuid_byindex(selected_index)
            else {
                return;
            };
            nv.verse_manifest_mut().change_current_verse_uuid(uuid);
            nv.set_state(NarraverseState::VerseAreaPick);
        }
        KeyCode::Char('e') => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = nv
                .get_verse_manifest()
                .current_verse_uuid_byindex(selected_index)
            else {
                return;
            };
            let Some(verse_edit) = VerseEdit::edit(nv.get_verse_manifest(), uuid) else {
                return;
            };
            nv.modals.verse_edit_modal = verse_edit;
            nv.set_state(NarraverseState::VerseEdit);
        }
        KeyCode::Char('n') => {
            nv.modals.verse_edit_modal = VerseEdit::default();
            nv.set_state(NarraverseState::VerseEdit);
        }
        KeyCode::Char('d') => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = nv
                .get_verse_manifest()
                .current_verse_uuid_byindex(selected_index)
            else {
                return;
            };
            nv.verse_manifest_mut().change_current_verse_uuid(uuid);
            nv.deletion_target = DeletionTarget::Verse;
            nv.set_state(NarraverseState::VerseDelete);
        }
        KeyCode::Char('c') => {
            nv.modals.config_edit_modal = ConfigEditModal::edit(nv.get_config());
            nv.set_state(NarraverseState::ConfigModal);
        }
        _ => {}
    }
}

fn selected_verse_uuid(verses: &VerseManifest, ind: usize) -> Option<Uuid> {
    let verse = verses.verses.get_index(ind)?;
    Some(*verse.0)
}
