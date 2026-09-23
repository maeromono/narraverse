use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::apcore::{
    apverse::{ApVerse, ApVerseState},
    decosutils::{bordered_block, center_area},
};

pub fn render_area_pick(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let Some(verse) = av.verses.verses.get(av.verses.verse_index) else {
        frame.render_widget(
            Paragraph::new("No 'Verse at this index.\nHit Esc to pick another one.")
                .red()
                .bold(),
            area,
        );
        return;
    };
    if av.cached_verse_name != verse.name {
        av.cached_verse_name = verse.name.clone();
    }
    let list = List::new(vec![
        "Narrative",
        "Entities",
        "Timeline",
        "Artifacts",
        "Edit 'Verse",
    ])
    .highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list.block(bordered_block("Select area to edit:")),
        center_area(area),
        &mut av.verse_area_pick_liststate,
    );
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => av.state = ApVerseState::VersePick,
        KeyCode::Char('j') | KeyCode::Down => av.verse_area_pick_liststate.select_next(),
        KeyCode::Char('k') | KeyCode::Up => av.verse_area_pick_liststate.select_previous(),
        KeyCode::Enter => av.state = match av.verse_area_pick_liststate.selected().unwrap() {
            0 => ApVerseState::NarrativePick,
            1 => ApVerseState::EntityPick,
            _ => todo!()
        },
        _ => {}
    }
}
