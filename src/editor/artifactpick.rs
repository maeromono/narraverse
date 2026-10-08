use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    widgets::{List, Paragraph},
};
use uuid::Uuid;

use crate::{
    editor::{
        artifacteditmodal::ArtifactEditModal, nveditor::EditorTarget, versedelete::DeletionTarget,
    },
    nvcore::{
        cacheutils::rebuild_art_list_cache, decosutils::{block_with_title, center_area}, narraverse::{Narraverse, NarraverseState}
    },
};

pub fn render_artifact_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!(
                "Verse with this UUID ({}) doesn't exist.",
                nv.verses.verse_uuid
            ),
            NarraverseState::VersePick,
        );
        return;
    };
    if verse.artifacts.len() == 0 {
        frame.render_widget(
            Paragraph::new("Hit n to create a new one.")
                .block(block_with_title("This Verse has no artifacts.")),
            center_area(area),
        );
        return;
    }
    let list = List::new(nv.cache.artifact_list_cache.iter().map(String::as_str))
        .highlight_style(Modifier::REVERSED)
        .block(block_with_title("Pick an artifact:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.verse_artifact_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => nv.state = NarraverseState::VerseAreaPick,
        KeyCode::Char('j') | KeyCode::Down => {
            nv.list_states.verse_artifact_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => nv
            .list_states
            .verse_artifact_pick_liststate
            .select_previous(),
        KeyCode::Char('g') | KeyCode::Home => {
            nv.list_states.verse_artifact_pick_liststate.select_first()
        }
        KeyCode::Char('G') | KeyCode::End => {
            nv.list_states.verse_artifact_pick_liststate.select_last()
        }
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
            nv.state = NarraverseState::Editor;
        }
        KeyCode::Char('n') => {
            nv.artifact_edit_modal = ArtifactEditModal::default();
            nv.state = NarraverseState::ArtifactModal;
        }
        KeyCode::Char('e') => {
            let Some(uuid) = selected_art_uuid(nv) else {
                return;
            };
            let Some(art_mod) = ArtifactEditModal::edit(&nv.verses, uuid) else {
                return;
            };
            nv.artifact_edit_modal = art_mod;
            nv.state = NarraverseState::ArtifactModal;
        }
        KeyCode::Char('d') => {
            let Some(uuid) = selected_art_uuid(nv) else {
                return;
            };
            nv.deletion_target = DeletionTarget::Artifact(uuid);
            nv.state = NarraverseState::VerseDelete;
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
    let Some(verse) = nv.verses.verses.get_mut(&nv.verses.verse_uuid) else {
        return;
    };
    let target_ind = if up { ind.checked_sub(1) } else { ind.checked_add(1) };
    let Some(target) = target_ind.filter(|&t| t < verse.artifacts.len()) else {
        return;
    };
    verse.artifacts.swap_indices(ind, target);
    rebuild_art_list_cache(nv);
    nv.list_states.verse_artifact_pick_liststate.select(Some(target));
}   
