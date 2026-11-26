use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, widgets::List};
use uuid::Uuid;

use crate::{
    editor::{
        artifacteditmodal::ArtifactEditModal, bindhelp::BindhelpMode, delete::DeletionTarget,
        nveditor::EditorTarget,
    },
    nvcore::narraverse::{Narraverse, NarraverseState},
    utils::{
        cacheutils::rebuild_art_list_cache,
        decosutils::{block_with_title, center_area, list_highlight},
        nvutils::enter_bindhelp,
        pickerutils::{handle_list_keys, render_list_empty_state},
    },
};

pub fn render_artifact_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let verse = nv.verses.current_verse();
    if verse.artifacts.len() == 0 {
        render_list_empty_state("artifacts", frame, area);
        return;
    }
    let list = list_highlight(List::new(
        nv.cache.artifact_list_cache.iter().map(String::as_str),
    ))
    .block(block_with_title("Pick an artifact:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.verse_artifact_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    if handle_list_keys(&mut nv.list_states.verse_artifact_pick_liststate, key) {
        return;
    }
    match key.code {
        KeyCode::Esc => nv.set_state(NarraverseState::VerseAreaPick),
        KeyCode::F(1) => enter_bindhelp(nv, BindhelpMode::Pickmenu),
        KeyCode::Enter => {
            let Some(uuid) = selected_art_uuid(nv) else {
                return;
            };
            if !nv
                .editor
                .add_buffer(&nv.verses, EditorTarget::Artifact, uuid)
            {
                return;
            };
            nv.set_state(NarraverseState::Editor);
        }
        KeyCode::Char('n') => {
            nv.modals.artifact_edit_modal = ArtifactEditModal::default();
            nv.set_state(NarraverseState::ArtifactModal);
        }
        KeyCode::Char('e') => {
            let Some(uuid) = selected_art_uuid(nv) else {
                return;
            };
            let Some(art_mod) = ArtifactEditModal::edit(&nv.verses, uuid) else {
                return;
            };
            nv.modals.artifact_edit_modal = art_mod;
            nv.set_state(NarraverseState::ArtifactModal);
        }
        KeyCode::Char('d') => {
            let Some(uuid) = selected_art_uuid(nv) else {
                return;
            };
            nv.deletion_target = DeletionTarget::Artifact(uuid);
            nv.set_state(NarraverseState::VerseDelete);
        }
        KeyCode::Char('J') => {
            let Some(ind) = nv.list_states.verse_artifact_pick_liststate.selected() else {
                return;
            };
            move_artifact(nv, ind, false);
        }
        KeyCode::Char('K') => {
            let Some(ind) = nv.list_states.verse_artifact_pick_liststate.selected() else {
                return;
            };
            move_artifact(nv, ind, true);
        }
        _ => {}
    }
}

fn selected_art_uuid(nv: &Narraverse) -> Option<Uuid> {
    let verse = nv.verses.verses.get(&nv.verses.verse_uuid)?;
    let index = nv.list_states.verse_artifact_pick_liststate.selected()?;
    verse
        .artifacts
        .get_index(index)
        .map(|(uuid, _)| uuid)
        .copied()
}

fn move_artifact(nv: &mut Narraverse, ind: usize, up: bool) {
    let verse = nv.verses.current_verse_mut();
    let target_ind = if up {
        ind.checked_sub(1)
    } else {
        ind.checked_add(1)
    };
    let Some(target) = target_ind.filter(|&t| t < verse.artifacts.len()) else {
        return;
    };
    verse.artifacts.swap_indices(ind, target);
    rebuild_art_list_cache(&mut nv.cache, &nv.verses);
    nv.list_states
        .verse_artifact_pick_liststate
        .select(Some(target));
}
