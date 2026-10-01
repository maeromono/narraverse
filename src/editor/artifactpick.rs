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
        nveditor::{EditorMode, NarraverseEditor},
        versedelete::DeletionTarget,
    },
    nvcore::{
        decosutils::{block_with_title, center_area},
        narraverse::{Narraverse, NarraverseState},
    },
};

pub fn render_artifact_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("Verse with this UUID ({}) doesn't exist.", nv.verses.verse_uuid),
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
    let mut artifact_list: Vec<String> = vec![];
    for artif in verse.artifacts.values() {
        artifact_list.push(format!(
            "[{}] {}",
            artif.artifact_type.name(),
            artif.name
        ));
    }
    let list = List::new(artifact_list)
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
        KeyCode::Char('k') | KeyCode::Up => {
            nv.list_states.verse_artifact_pick_liststate.select_previous()
        }
        KeyCode::Enter => {
            let Some(uuid) = selected_art_uuid(nv) else {
                return;
            };
            let Some(nar_ed) = NarraverseEditor::edit(&nv.verses, EditorMode::Artifact, uuid) else {
                return;
            };
            nv.editor = nar_ed;
            nv.state = NarraverseState::Editor;
        }
        KeyCode::Char('n') => {
            todo!()
        }
        KeyCode::Char('e') => {
            todo!()
        }
        KeyCode::Char('d') => {
            let Some(_) = nv.list_states.verse_artifact_pick_liststate.selected() else {
                return;
            };
            nv.deletion_target = DeletionTarget::Entity;
            nv.state = NarraverseState::VerseDelete;
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
