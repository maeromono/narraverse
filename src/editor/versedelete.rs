use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

use crate::nvcore::{
    decosutils::center_area,
    narraverse::{Narraverse, NarraverseState},
    save::save_verses,
};

#[derive(PartialEq, Eq)]
pub enum DeletionTarget {
    Verse,
    Narrative,
    Entity,
}

pub fn render_delete_screen(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    let delete_confirmation_message: Paragraph = match nv.deletion_target {
        DeletionTarget::Verse => {
            Paragraph::new(format!(
                "Delete Verse \"{}\"?\nAll narrative you've written and entities you've created will be permanently erased.",
                verse.name
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()
        }
        DeletionTarget::Narrative => {
            let Some(narrative_index) = nv.list_states.verse_narrative_pick_liststate.selected()
            else {
                nv.state = NarraverseState::NarrativePick;
                return;
            };
            let Some(narrative) = verse.narratives.get(narrative_index) else {
                nv.send_error(
                    String::from("No narrative at this index."),
                    NarraverseState::NarrativePick,
                );
                return;
            };
            Paragraph::new(format!(
                "Delete narrative \"{}\"?\nYou cannot revert this.",
                narrative.title
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()
        }
        DeletionTarget::Entity => {
            let Some(entity_index) = nv.list_states.verse_entity_pick_liststate.selected() else {
                nv.state = NarraverseState::EntityPick;
                return;
            };
            let Some(entity) = verse.entities.get(entity_index) else {
                nv.send_error(String::from("No entity at this index."), NarraverseState::EntityPick);
                return;
            };
            Paragraph::new(format!("Delete entity \"{}\"?\nYou cannot revert this.", entity.name)).wrap(Wrap { trim: false }).yellow().bold()
        }
    };
    frame.render_widget(delete_confirmation_message, center_area(area));
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('n') => {
            nv.state = match nv.deletion_target {
                DeletionTarget::Verse => NarraverseState::VersePick,
                DeletionTarget::Narrative => NarraverseState::NarrativePick,
                DeletionTarget::Entity => NarraverseState::EntityPick,
            };
        }
        KeyCode::Char('y') => {
            confirm_delete(nv);
        }
        _ => {}
    }
}

fn confirm_delete(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get_mut(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    match nv.deletion_target {
        DeletionTarget::Verse => {
            let mut verse_index = nv.verses.verse_index;
            nv.verses.verses.remove(verse_index);
            if nv.verses.verses.len() == 0 {
                nv.list_states.verse_pick_liststate.select(None);
            } else if verse_index >= nv.verses.verses.len() {
                verse_index -= 1;
                nv.verses.verse_index = verse_index;
            }
            nv.state = NarraverseState::VersePick;
        }
        DeletionTarget::Narrative => {
            let Some(narrative_index) = nv.list_states.verse_narrative_pick_liststate.selected()
            else {
                nv.state = NarraverseState::NarrativePick;
                return;
            };
            verse.narratives.remove(narrative_index);
            nv.list_states.verse_narrative_pick_liststate.select(None);
            nv.state = NarraverseState::NarrativePick;
        }
        DeletionTarget::Entity => {
            let Some(entity_index) = nv.list_states.verse_entity_pick_liststate.selected() else {
                nv.state = NarraverseState::EntityPick;
                return;
            };
            verse.entities.remove(entity_index);
            nv.list_states.verse_entity_pick_liststate.select(None);
            nv.state = NarraverseState::EntityPick;
        }
    }
    if let Err(err) = save_verses(&mut nv.verses) {
        nv.send_error(
            format!("Cannot finalize deletion:\n{}", err),
            NarraverseState::VersePick,
        );
    }
}
