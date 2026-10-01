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
        narrativeeditmodal::NarrativeEditModal,
        nveditor::{EditorMode, NarraverseEditor},
        versedelete::DeletionTarget,
    },
    nvcore::{
        decosutils::{block_with_title, center_area},
        narraverse::{Narraverse, NarraverseState},
    },
};

pub fn render_narrative_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    if verse.narratives.len() == 0 {
        frame.render_widget(
            Paragraph::new("Hit n to create a new one.")
                .block(block_with_title("This Verse has no narrative.")),
            center_area(area),
        );
        return;
    }
    let mut narrative_list: Vec<String> = vec![];
    for narr in verse.narratives.values() {
        narrative_list.push(narr.title.clone());
    }
    let list = List::new(narrative_list)
        .highlight_style(Modifier::REVERSED)
        .block(block_with_title("Pick a narrative:"));
    frame.render_stateful_widget(
        list,
        center_area(area),
        &mut nv.list_states.verse_narrative_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => nv.state = NarraverseState::VerseAreaPick,
        KeyCode::Char('j') | KeyCode::Down => {
            nv.list_states.verse_narrative_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => nv
            .list_states
            .verse_narrative_pick_liststate
            .select_previous(),
        KeyCode::Enter => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            let Some(nar_ed) = NarraverseEditor::edit(&nv.verses, EditorMode::Narrative, uuid)
            else {
                return;
            };
            nv.editor = nar_ed;
            nv.state = NarraverseState::Editor;
        }
        KeyCode::Char('n') => {
            nv.narrative_edit_modal = NarrativeEditModal::default().prefill_writtenby_field(nv);
            nv.state = NarraverseState::NarrativeModal;
        }
        KeyCode::Char('e') => {
            let Some(index) = nv.list_states.verse_narrative_pick_liststate.selected() else {
                return;
            };
            let Some(modal) = NarrativeEditModal::edit(nv, index) else {
                return;
            };
            nv.narrative_edit_modal = modal;
            nv.state = NarraverseState::NarrativeModal;
        }
        KeyCode::Char('d') => {
            let Some(_) = nv.list_states.verse_narrative_pick_liststate.selected() else {
                return;
            };
            nv.deletion_target = DeletionTarget::Narrative;
            nv.state = NarraverseState::VerseDelete;
        }
        _ => {}
    }
}

fn selected_narr_uuid(nv: &Narraverse) -> Option<Uuid> {
    let verse = nv.verses.verses.get(nv.verses.verse_index)?;
    let index = nv.list_states.verse_narrative_pick_liststate.selected()?;
    verse
        .narratives
        .get_index(index)
        .map(|(uuid, _)| uuid)
        .copied()
}
