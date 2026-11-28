use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, widgets::List};

use crate::{
    editor::{delete::DeletionTarget, verseedit::VerseEdit},
    nvcore::narraverse::{Narraverse, NarraverseState},
    utils::{
        decosutils::{block_with_title, center_area, list_highlight},
        pickerutils::handle_list_keys,
    },
};

pub fn render_area_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let list = list_highlight(List::new(vec![
        "Narrative",
        "Entities",
        "Artifacts",
        "Edit Verse",
        "Delete Verse",
    ]));
    frame.render_stateful_widget(
        list.block(block_with_title("Select an area:")),
        center_area(area),
        &mut nv.list_states.verse_area_pick_liststate,
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    if handle_list_keys(&mut nv.list_states.verse_area_pick_liststate, key) {
        return;
    }
    match key.code {
        KeyCode::Char('q') => nv.set_state(NarraverseState::Quit),
        KeyCode::Esc => {
            // cross verse buffers probably won't be possible because of how narraverse is built.
            nv.editor.purge_all_buffers();
            nv.list_states.verse_narrative_pick_liststate.select(None);
            nv.list_states.verse_entity_pick_liststate.select(None);
            nv.list_states.verse_artifact_pick_liststate.select(None);
            nv.set_state(NarraverseState::VersePick);
        }
        KeyCode::Enter => {
            let Some(selected_index) = nv.list_states.verse_area_pick_liststate.selected() else {
                return;
            };
            let uuid = nv.get_verse_manifest().verse_uuid;
            let next = match selected_index {
                0 => NarraverseState::NarrativePick,
                1 => NarraverseState::EntityPick,
                2 => NarraverseState::ArtifactPick,
                3 => {
                    let Some(verse_edit) = VerseEdit::edit(nv.get_verse_manifest(), uuid) else {
                        return;
                    };
                    nv.modals.verse_edit_modal = verse_edit;
                    NarraverseState::VerseEdit
                }
                4 => {
                    nv.deletion_target = DeletionTarget::Verse;
                    NarraverseState::VerseDelete
                }
                _ => todo!(),
            };
            nv.set_state(next);
        }
        _ => {}
    }
}
