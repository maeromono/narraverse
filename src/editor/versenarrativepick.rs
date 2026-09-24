use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::{
    apcore::{
        apverse::{ApVerse, ApVerseState},
        decosutils::{bordered_block, center_area},
    },
    editor::{narrativeedit::NarrativeEditor, narrativeeditmodal::NarrativeEditModal},
};

pub fn render_narrative_pick(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let Some(verse) = av.verses.verses.get(av.verses.verse_index) else {
        frame.render_widget(
            Paragraph::new("No 'Verse at this index.\nHit Esc to pick another one.")
                .red()
                .bold(),
            area,
        );
        return;
    };
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
        &mut av.list_states.verse_narrative_pick_liststate,
    );
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => av.state = ApVerseState::VerseAreaPick,
        KeyCode::Char('j') | KeyCode::Down => {
            av.list_states.verse_narrative_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => av
            .list_states
            .verse_narrative_pick_liststate
            .select_previous(),
        KeyCode::Enter => {
            let index = av
                .list_states
                .verse_narrative_pick_liststate
                .selected()
                .unwrap();
            av.narrative_editor = NarrativeEditor::edit(av, index);
            av.state = ApVerseState::NarrativeEdit;
        }
        KeyCode::Char('n') => {
            av.narrative_edit_modal = NarrativeEditModal::default();
            av.state = ApVerseState::NewNarrative;
        }
        KeyCode::Char('r') => {
            let index = av
                .list_states
                .verse_narrative_pick_liststate
                .selected()
                .unwrap();
            let Some(modal) = NarrativeEditModal::edit(av, index) else {
                return;
            };
            av.narrative_edit_modal = modal;
            av.state = ApVerseState::NewNarrative;
        }
        _ => {}
    }
}
