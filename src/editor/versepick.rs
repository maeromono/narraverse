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

pub fn render_worldpick(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let mut verse_list: Vec<String> = vec![];
    if av.verses.verses.len() == 0 {
        frame.render_widget(Paragraph::new("No verses").red().bold(), area);
        return;
    }
    for verse in &av.verses.verses {
        verse_list.push(format!("Verse: {}", verse.name));
    }
    let list = List::new(verse_list).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list.block(bordered_block("Pick a 'Verse:")),
        center_area(area),
        &mut av.list_states.verse_pick_liststate,
    );
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => av.state = ApVerseState::Quit,
        KeyCode::Char('j') | KeyCode::Down => av.list_states.verse_pick_liststate.select_next(),
        KeyCode::Char('k') | KeyCode::Up => av.list_states.verse_pick_liststate.select_previous(),
        KeyCode::Enter => {
            av.verses.verse_index = av.list_states.verse_pick_liststate.selected().unwrap();
            av.state = ApVerseState::VerseAreaPick;
        }
        _ => {}
    }
}
