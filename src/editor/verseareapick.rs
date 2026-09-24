use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::nvcore::{
    narraverse::{Narraverse, NarraverseState},
    decosutils::{bordered_block, center_area},
};

pub fn render_area_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        frame.render_widget(
            Paragraph::new("No 'Verse at this index.\nHit Esc to pick another one.")
                .red()
                .bold(),
            area,
        );
        return;
    };
    if nv.cached_verse_name != verse.name {
        nv.cached_verse_name = verse.name.clone();
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
        &mut nv.list_states.verse_area_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => nv.state = NarraverseState::VersePick,
        KeyCode::Char('j') | KeyCode::Down => {
            nv.list_states.verse_area_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => {
            nv.list_states.verse_area_pick_liststate.select_previous()
        }
        KeyCode::Enter => {
            nv.state = match nv.list_states.verse_area_pick_liststate.selected().unwrap() {
                0 => NarraverseState::NarrativePick,
                1 => NarraverseState::EntityPick,
                _ => todo!(),
            }
        }
        _ => {}
    }
}
