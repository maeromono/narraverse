use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{List, Paragraph, Wrap},
};
use uuid::Uuid;

use crate::{
    editor::{
        bindhelp::BindhelpMode, delete::DeletionTarget, narrativeeditmodal::NarrativeEditModal,
        nveditor::EditorTarget,
    },
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        versemanifest::Verse,
    },
    utils::{
        decosutils::{block_with_title, center_area, list_highlight},
        nvutils::enter_bindhelp,
        pickerutils::{handle_list_keys, narrative_pick_items, render_list_empty_state},
    },
};

pub fn render_narrative_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let manifest = nv.get_verse_manifest();
    if manifest.current_verse().narratives.is_empty() {
        render_list_empty_state("narratives", frame, area);
        return;
    }
    let narr_pick_layout =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(7)]).split(center_area(area));
    let list = list_highlight(List::new(narrative_pick_items(manifest)))
        .block(block_with_title("Pick a narrative:"));
    frame.render_stateful_widget(
        list,
        narr_pick_layout[0],
        &mut nv.list_states.verse_narrative_pick_liststate,
    );
    let verse = nv.get_verse_manifest().current_verse(); // if i use the manifest here the compiler will execute me.
    display_narr_info(&nv, verse, frame, narr_pick_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    if handle_list_keys(&mut nv.list_states.verse_narrative_pick_liststate, key) {
        return;
    }
    match key.code {
        KeyCode::Esc => nv.set_state(NarraverseState::VerseAreaPick),
        KeyCode::F(1) => enter_bindhelp(nv, BindhelpMode::Pickmenu),
        KeyCode::Enter => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            if !nv.add_editor_buffer(EditorTarget::Narrative, uuid) {
                return;
            };
            nv.set_state(NarraverseState::Editor);
        }
        KeyCode::Char('n') => {
            nv.modals.narrative_edit_modal =
                NarrativeEditModal::default().prefill_writtenby_field(nv);
            nv.set_state(NarraverseState::NarrativeModal);
        }
        KeyCode::Char('e') => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            let Some(modal) = NarrativeEditModal::edit(nv.get_verse_manifest(), uuid) else {
                return;
            };
            nv.modals.narrative_edit_modal = modal;
            nv.set_state(NarraverseState::NarrativeModal);
        }
        KeyCode::Char('d') => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            nv.deletion_target = DeletionTarget::Narrative(uuid);
            nv.set_state(NarraverseState::VerseDelete);
        }
        _ => {}
    }
}

fn display_narr_info(nv: &Narraverse, verse: &Verse, frame: &mut Frame, area: Rect) {
    let Some(uuid) = selected_narr_uuid(nv) else {
        frame.render_widget(
            Paragraph::new("Pick a narrative to display its info here."),
            area,
        );
        return;
    };
    let Some(narr) = verse.narratives.get(&uuid) else {
        return;
    };
    frame.render_widget(
        Paragraph::new(format!(
            "{}\nWritten by: {}\n{}",
            narr.title,
            narr.written_by,
            narr.summary
                .clone()
                .unwrap_or(String::from("Narrative has no summary."))
        ))
        .wrap(Wrap { trim: true }),
        area,
    );
}

fn selected_narr_uuid(nv: &Narraverse) -> Option<Uuid> {
    let index = nv.list_states.verse_narrative_pick_liststate.selected()?;
    nv.get_verse_manifest()
        .current_verse()
        .narratives
        .get_index(index)
        .map(|(uuid, _)| *uuid)
}
