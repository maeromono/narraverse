use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{List, Paragraph, Wrap},
};
use uuid::Uuid;

use crate::{
    editor::{
        bindhelp::BindhelpMode, delete::DeletionTarget, narrativeeditmodal::NarrativeEditModal, nveditor::EditorTarget
    },
    nvcore::{
        narraverse::{Narraverse, NarraverseState}, verses::Verse
    }, utils::{decosutils::{block_with_title, center_area, list_highlight}, nvutils::enter_bindhelp},
};

pub fn render_narrative_pick(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    if verse.narratives.len() == 0 {
        frame.render_widget(
            Paragraph::new("Hit n to create a new one.")
                .block(block_with_title("This Verse has no narrative.")),
            center_area(area),
        );
        return;
    }
    let narr_pick_layout =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(7)]).split(center_area(area));
    let list = list_highlight(List::new(
        nv.cache.narrative_list_cache.iter().map(String::as_str),
    ))
    .block(block_with_title("Pick a narrative:"));
    frame.render_stateful_widget(
        list,
        narr_pick_layout[0],
        &mut nv.list_states.verse_narrative_pick_liststate,
    );
    display_narr_info(&nv, verse, frame, narr_pick_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => nv.state = NarraverseState::VerseAreaPick,
        KeyCode::F(1) => enter_bindhelp(nv, BindhelpMode::Pickmenu),
        KeyCode::Char('j') | KeyCode::Down => {
            nv.list_states.verse_narrative_pick_liststate.select_next()
        }
        KeyCode::Char('k') | KeyCode::Up => nv
            .list_states
            .verse_narrative_pick_liststate
            .select_previous(),
        KeyCode::Char('g') | KeyCode::Home => {
            nv.list_states.verse_narrative_pick_liststate.select_first()
        }
        KeyCode::Char('G') | KeyCode::End => {
            nv.list_states.verse_narrative_pick_liststate.select_last()
        }
        KeyCode::Enter => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            if !nv
                .editor
                .add_buffer(&nv.verses, EditorTarget::Narrative, uuid)
            {
                return;
            };
            nv.state = NarraverseState::Editor;
        }
        KeyCode::Char('n') => {
            nv.modals.narrative_edit_modal = NarrativeEditModal::default().prefill_writtenby_field(nv);
            nv.state = NarraverseState::NarrativeModal;
        }
        KeyCode::Char('e') => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            let Some(modal) = NarrativeEditModal::edit(&nv.verses, uuid) else {
                return;
            };
            nv.modals.narrative_edit_modal = modal;
            nv.state = NarraverseState::NarrativeModal;
        }
        KeyCode::Char('d') => {
            let Some(uuid) = selected_narr_uuid(nv) else {
                return;
            };
            nv.deletion_target = DeletionTarget::Narrative(uuid);
            nv.state = NarraverseState::VerseDelete;
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
    let verse = nv.verses.verses.get(&nv.verses.verse_uuid)?;
    let index = nv.list_states.verse_narrative_pick_liststate.selected()?;
    verse
        .narratives
        .get_index(index)
        .map(|(uuid, _)| uuid)
        .copied()
}
