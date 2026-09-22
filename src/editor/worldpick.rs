use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::apcore::apverse::{ApVerse, ApVerseState};

pub fn render_worldpick(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let mut verse_list: Vec<String> = vec![];
    if av.verses.len() == 0 {
        frame.render_widget(Paragraph::new("No verses").red().bold(), area);
        return;
    }
    for verse in &av.verses {
        verse_list.push(format!("Verse: {}", verse.name));
    }
    let list = List::new(verse_list).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list,
        area.centered(Constraint::Max(75), Constraint::Percentage(30)),
        &mut av.verse_pick_liststate,
    );
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => av.state = ApVerseState::Quit,
        KeyCode::Char('j') | KeyCode::Down => av.verse_pick_liststate.select_next(),
        KeyCode::Char('k') | KeyCode::Up => av.verse_pick_liststate.select_previous(),
        _ => {}
    }
}
