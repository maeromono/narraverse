use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::List,
};
use uuid::Uuid;

use crate::{
    editor::{bindhelp::BindhelpMode, delete::DeletionTarget, entityedit::EntityEditModal, nveditor::EditorTarget},
    nvcore::narraverse::{Narraverse, NarraverseState}, utils::{cacheutils::rebuild_ent_list_cache, decosutils::{block_with_title, center_area, list_highlight}, nvutils::enter_bindhelp, pickerutils::{handle_list_keys, render_list_empty_state}},
};

pub fn render_entity_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    if verse.entities.len() == 0 {
        render_list_empty_state("entities", frame, area);
        return;
    }
    let list = list_highlight(List::new(
        nv.cache.entity_list_cache.iter().map(String::as_str),
    ))
    .block(block_with_title("Pick an entity:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.verse_entity_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    if handle_list_keys(&mut nv.list_states.verse_entity_pick_liststate, key) {
        return;
    }
    match key.code {
        KeyCode::Esc => nv.state = NarraverseState::VerseAreaPick,
        KeyCode::F(1) => enter_bindhelp(nv, BindhelpMode::Pickmenu),
        KeyCode::Enter => {
            let Some(uuid) = selected_ent_uuid(nv) else {
                return;
            };
            if !nv.editor.add_buffer(&nv.verses, EditorTarget::Entity, uuid) {
                return;
            };
            nv.state = NarraverseState::Editor;
        }
        KeyCode::Char('n') => {
            nv.modals.entity_edit_modal = EntityEditModal::default();
            nv.state = NarraverseState::EntityModal;
        }
        KeyCode::Char('e') => {
            let Some(uuid) = selected_ent_uuid(nv) else {
                return;
            };
            let Some(modal) = EntityEditModal::edit(&nv.verses, uuid) else {
                return;
            };
            nv.modals.entity_edit_modal = modal;
            nv.state = NarraverseState::EntityModal;
        }
        KeyCode::Char('d') => {
            let Some(uuid) = selected_ent_uuid(nv) else {
                return;
            };
            nv.deletion_target = DeletionTarget::Entity(uuid);
            nv.state = NarraverseState::VerseDelete;
        }
        KeyCode::Char('J') => {
            let Some(ind) = nv.list_states.verse_entity_pick_liststate.selected() else {
                return;
            };
            move_entity(nv, ind, false);
        }
        KeyCode::Char('K') => {
            let Some(ind) = nv.list_states.verse_entity_pick_liststate.selected() else {
                return;
            };
            move_entity(nv, ind, true);
        }
        _ => {}
    }
}

fn selected_ent_uuid(nv: &Narraverse) -> Option<Uuid> {
    let verse = nv.verses.verses.get(&nv.verses.verse_uuid)?;
    let index = nv.list_states.verse_entity_pick_liststate.selected()?;
    verse
        .entities
        .get_index(index)
        .map(|(uuid, _)| uuid)
        .copied()
}

fn move_entity(nv: &mut Narraverse, ind: usize, up: bool) {
    let Some(verse) = nv.verses.verses.get_mut(&nv.verses.verse_uuid) else {
        return;
    };
    let target_ind = if up { ind.checked_sub(1) } else { ind.checked_add(1) };
    let Some(target) = target_ind.filter(|&t| t < verse.entities.len()) else {
        return;
    };
    verse.entities.swap_indices(ind, target);
    rebuild_ent_list_cache(&mut nv.cache, &nv.verses);
    nv.list_states.verse_entity_pick_liststate.select(Some(target));
}   
