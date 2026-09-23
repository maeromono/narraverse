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
    for narr in &verse.narrative {
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
        KeyCode::Char('q') => av.state = ApVerseState::Quit,
        KeyCode::Char('j') | KeyCode::Down => {
            av.list_states.verse_narrative_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => av
            .list_states
            .verse_narrative_pick_liststate
            .select_previous(),
        KeyCode::Enter => {
            av.verses.verse_index = av
                .list_states
                .verse_narrative_pick_liststate
                .selected()
                .unwrap();
            av.state = ApVerseState::NarrativeEdit;
        }
        _ => {}
    }
}
