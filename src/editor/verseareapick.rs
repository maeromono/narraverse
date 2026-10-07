use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::Modifier, widgets::List};

use crate::{
    editor::verseedit::VerseEdit,
    nvcore::{
        decosutils::{block_with_title, center_area},
        narraverse::{Narraverse, NarraverseState},
    },
};

pub fn render_area_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    if nv.cached_verse_name != verse.name {
        nv.cached_verse_name = verse.name.clone();
    }
    let list = List::new(vec!["Narrative", "Entities", "Artifacts", "Edit Verse"])
        .highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list.block(block_with_title("Select area to edit:")),
        center_area(area),
        &mut nv.list_states.verse_area_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => nv.state = NarraverseState::Quit,
        KeyCode::Esc => {
            // cross verse buffers probably won't be possible because of how narraverse is built.
            nv.editor.purge_all_buffers();
            nv.state = NarraverseState::VersePick;
        },
        KeyCode::Char('j') | KeyCode::Down => {
            nv.list_states.verse_area_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => {
            nv.list_states.verse_area_pick_liststate.select_previous()
        }
        KeyCode::Enter => {
            let Some(selected_index) = nv.list_states.verse_area_pick_liststate.selected() else {
                return;
            };
            let Some(verse_edit) = VerseEdit::edit(&nv.verses, nv.verses.verse_uuid) else {
                return;
            };
            nv.verse_edit_modal = verse_edit;
            nv.state = match selected_index {
                0 => NarraverseState::NarrativePick,
                1 => NarraverseState::EntityPick,
                2 => NarraverseState::ArtifactPick,
                3 => NarraverseState::VerseEdit,
                _ => todo!(),
            }
        }
        _ => {}
    }
}
