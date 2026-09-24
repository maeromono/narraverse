use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::{
    editor::{versedelete::DeletionTarget, verseedit::VerseEdit},
    nvcore::{
        decosutils::{bordered_block, center_area},
        narraverse::{Narraverse, NarraverseState},
    },
};

pub fn render_worldpick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let mut verse_list: Vec<String> = vec![];
    if nv.verses.verses.len() == 0 {
        frame.render_widget(Paragraph::new("No verses").red().bold(), area);
        return;
    }
    for verse in &nv.verses.verses {
        verse_list.push(format!("Verse: {}", verse.name));
    }
    let list = List::new(verse_list).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list.block(bordered_block("Pick a 'Verse:")),
        center_area(area),
        &mut nv.list_states.verse_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => nv.state = NarraverseState::Quit,
        KeyCode::Char('j') | KeyCode::Down => nv.list_states.verse_pick_liststate.select_next(),
        KeyCode::Char('k') | KeyCode::Up => nv.list_states.verse_pick_liststate.select_previous(),
        KeyCode::Enter => {
            nv.verses.verse_index = nv.list_states.verse_pick_liststate.selected().unwrap();
            nv.state = NarraverseState::VerseAreaPick;
        }
        KeyCode::Char('e') => {
            nv.verses.verse_index = nv.list_states.verse_pick_liststate.selected().unwrap();
            let Some(verse_edit) = VerseEdit::edit(nv, nv.verses.verse_index) else {
                return;
            };
            nv.verse_editor = verse_edit;
            nv.state = NarraverseState::VerseEdit;
        }
        KeyCode::Char('n') => {
            nv.verse_editor = VerseEdit::default();
            nv.state = NarraverseState::VerseEdit;
        }
        KeyCode::Char('d') => {
            nv.verses.verse_index = nv.list_states.verse_pick_liststate.selected().unwrap();
            nv.deletion_target = DeletionTarget::Verse;
            nv.state = NarraverseState::VerseDelete;
        }
        _ => {}
    }
}
