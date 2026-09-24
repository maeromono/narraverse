use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::{
    editor::{
        narrativeedit::NarrativeEditor, narrativeeditmodal::NarrativeEditModal,
        versedelete::DeletionTarget,
    },
    nvcore::{
        decosutils::{bordered_block, center_area},
        narraverse::{Narraverse, NarraverseState},
    },
};

pub fn render_narrative_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        frame.render_widget(
            Paragraph::new("No 'Verse at this index.\nHit Esc to pick another one.")
                .red()
                .bold(),
            area,
        );
        return;
    };
    if verse.narratives.len() == 0 {
        nv.narrative_edit_modal = NarrativeEditModal::default();
        nv.state = NarraverseState::NewNarrative;
        return;
    }
    let mut narrative_list: Vec<String> = vec![];
    for narr in &verse.narratives {
        narrative_list.push(narr.title.clone());
    }
    let list = List::new(narrative_list)
        .highlight_style(Modifier::REVERSED)
        .block(bordered_block("Pick a narrative:"));
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
            let index = nv
                .list_states
                .verse_narrative_pick_liststate
                .selected()
                .unwrap();
            nv.narrative_editor = NarrativeEditor::edit(nv, index);
            nv.state = NarraverseState::NarrativeEdit;
        }
        KeyCode::Char('n') => {
            nv.narrative_edit_modal = NarrativeEditModal::default();
            nv.state = NarraverseState::NewNarrative;
        }
        KeyCode::Char('r') => {
            let index = nv
                .list_states
                .verse_narrative_pick_liststate
                .selected()
                .unwrap();
            let Some(modal) = NarrativeEditModal::edit(nv, index) else {
                return;
            };
            nv.narrative_edit_modal = modal;
            nv.state = NarraverseState::NewNarrative;
        }
        KeyCode::Char('d') => {
            nv.deletion_target = DeletionTarget::Narrative;
            nv.state = NarraverseState::VerseDelete;
        }
        _ => {}
    }
}
