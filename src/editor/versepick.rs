use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    widgets::{List, Paragraph},
};
use uuid::Uuid;

use crate::{
    editor::{versedelete::DeletionTarget, verseedit::VerseEdit},
    nvcore::{
        cacheutils::rebuild_narr_list_cache, decosutils::{block_with_title, center_area}, narraverse::{Narraverse, NarraverseState}, verses::NarraverseVerses
    },
};

pub fn render_worldpick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let mut verse_list: Vec<String> = vec![];
    if nv.verses.verses.len() == 0 {
        frame.render_widget(
            Paragraph::new("Hit n to create a new one.")
                .block(block_with_title("You have no Verses.")),
            center_area(area),
        );
        return;
    }
    for verse in nv.verses.verses.values() {
        verse_list.push(format!("Verse: {}", verse.name));
    }
    let list = List::new(verse_list).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list.block(block_with_title("Pick a 'Verse:")),
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
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = selected_verse_uuid(&nv.verses, selected_index) else {
                return;
            };
            nv.verses.verse_uuid = uuid;
            nv.state = NarraverseState::VerseAreaPick;
            rebuild_narr_list_cache(nv);
        }
        KeyCode::Char('e') => {
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = selected_verse_uuid(&nv.verses, selected_index) else {
                return;
            };
            let Some(verse_edit) = VerseEdit::edit(&nv.verses, uuid) else {
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
            let Some(selected_index) = nv.list_states.verse_pick_liststate.selected() else {
                return;
            };
            let Some(uuid) = selected_verse_uuid(&nv.verses, selected_index) else {
                return;
            };
            nv.verses.verse_uuid = uuid;
            nv.deletion_target = DeletionTarget::Verse;
            nv.state = NarraverseState::VerseDelete;
        }
        _ => {}
    }
}

fn selected_verse_uuid(verses: &NarraverseVerses, ind: usize) -> Option<Uuid> {
    let verse = verses.verses.get_index(ind)?;
    Some(*verse.0)
}
