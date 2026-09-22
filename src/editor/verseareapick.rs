use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::Modifier, widgets::List};

use crate::apcore::{
    apverse::{ApVerse, ApVerseState},
    decosutils::{bordered_block, center_area},
};

pub fn render_area_pick(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let list = List::new(vec![
        "Narrative",
        "Entities",
        "Edit 'Verse",
        "Something else",
        "sjfkljhfjlsaja",
    ]).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list.block(bordered_block("Select area to edit:")),
        center_area(area),
        &mut av.verse_area_pick_liststate,
    );
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => av.state = ApVerseState::VerseAreaPick,
        KeyCode::Char('j') | KeyCode::Down => av.verse_area_pick_liststate.select_next(),
        KeyCode::Char('k') | KeyCode::Up => av.verse_area_pick_liststate.select_previous(),
        _ => {}
    }
}
