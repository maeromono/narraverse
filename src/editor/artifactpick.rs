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
        decosutils::{block_with_title, center_area, list_highlight},
        nvutils::enter_bindhelp,
        pickerutils::{artifact_pick_items, handle_list_keys, render_list_empty_state},
    },
};

pub fn render_artifact_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let manifest = nv.get_verse_manifest();
    if manifest.current_verse().artifacts.is_empty() {
        render_list_empty_state("artifacts", frame, area);
        return;
    }
    let list = list_highlight(List::new(artifact_pick_items(manifest)))
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
            if !nv.add_editor_buffer(EditorTarget::Artifact, uuid) {
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
            let Some(art_mod) = ArtifactEditModal::edit(nv.get_verse_manifest(), uuid) else {
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
    let index = nv.list_states.verse_artifact_pick_liststate.selected()?;
    nv.get_verse_manifest()
        .current_verse()
        .artifacts
        .get_index(index)
        .map(|(uuid, _)| *uuid)
}

fn move_artifact(nv: &mut Narraverse, ind: usize, up: bool) {
    let verse = nv.verse_manifest_mut().current_verse_mut();
    let target_ind = if up {
        ind.checked_sub(1)
    } else {
        ind.checked_add(1)
    };
    let Some(target) = target_ind.filter(|&t| t < verse.artifacts.len()) else {
        return;
    };
    verse.artifacts.swap_indices(ind, target);
    nv.list_states
        .verse_artifact_pick_liststate
        .select(Some(target));
}
