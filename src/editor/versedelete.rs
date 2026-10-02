use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Wrap},
};
use uuid::Uuid;

use crate::nvcore::{
    cacheutils::rebuild_narr_list_cache,
    decosutils::center_area,
    narraverse::{Narraverse, NarraverseState},
    save::save_verses,
};

#[derive(PartialEq, Eq)]
pub enum DeletionTarget {
    Verse,
    Narrative(Uuid),
    Entity(Uuid),
    Artifact(Uuid),
}

pub fn render_delete_screen(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    let delete_confirmation_message: Paragraph = match nv.deletion_target {
        DeletionTarget::Verse => Paragraph::new(format!(
            "Delete Verse \"{}\"?\nIt'll be removed from the index, but will remain on disk.",
            verse.name
        ))
        .wrap(Wrap { trim: false })
        .yellow()
        .bold(),
        DeletionTarget::Narrative(uuid) => {
            let Some(narrative) = verse.narratives.get(&uuid) else {
                nv.send_error(
                    format!("Narrative with this UUID ({}) doesn't exist.", uuid),
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
        DeletionTarget::Entity(uuid) => {
            let Some(entity) = verse.entities.get(&uuid) else {
                nv.send_error(
                    format!("Entity with this UUID ({}) doesn't exist.", uuid),
                    NarraverseState::EntityPick,
                );
                return;
            };
            Paragraph::new(format!(
                "Delete entity \"{}\"?\nYou cannot revert this.",
                entity.name
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()
        }
        DeletionTarget::Artifact(uuid) => {
            let Some(artifact) = verse.artifacts.get(&uuid) else {
                nv.send_error(
                    format!("Artifact with this UUID ({}) doesn't exist.", uuid),
                    NarraverseState::ArtifactPick,
                );
                return;
            };
            Paragraph::new(format!(
                "Delete artifact \"{}\"?\nYou cannot revert this.",
                artifact.name
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()
        }
    };
    frame.render_widget(delete_confirmation_message, center_area(area));
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('n') => {
            nv.state = match nv.deletion_target {
                DeletionTarget::Verse => NarraverseState::VersePick,
                DeletionTarget::Narrative(_) => NarraverseState::NarrativePick,
                DeletionTarget::Entity(_) => NarraverseState::EntityPick,
                DeletionTarget::Artifact(_) => NarraverseState::ArtifactPick,
            };
        }
        KeyCode::Char('y') => {
            confirm_delete(nv);
        }
        _ => {}
    }
}

fn confirm_delete(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get_mut(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    match nv.deletion_target {
        DeletionTarget::Verse => {
            let verse_uuid = nv.verses.verse_uuid;
            nv.verses.verses.shift_remove(&verse_uuid);
            nv.list_states.verse_pick_liststate.select(None);
            nv.state = NarraverseState::VersePick;
            nv.send_footer_notification(String::from("Verse deleted"));
        }
        DeletionTarget::Narrative(uuid) => {
            if !verse.narratives.contains_key(&uuid) {
                nv.send_error(
                    format!(
                        "Couldn't delete narrative with UUID {}.\nPerhaps it was already deleted.",
                        uuid
                    ),
                    NarraverseState::NarrativePick,
                );
                return;
            }
            verse.narratives.shift_remove(&uuid);
            nv.list_states.verse_narrative_pick_liststate.select(None);
            rebuild_narr_list_cache(nv);
            nv.state = NarraverseState::NarrativePick;
            nv.send_footer_notification(String::from("Narrative deleted"));
        }
        DeletionTarget::Entity(uuid) => {
            if !verse.entities.contains_key(&uuid) {
                nv.send_error(
                    format!(
                        "Couldn't delete entity with UUID {}.\nPerhaps it was already deleted.",
                        uuid
                    ),
                    NarraverseState::EntityPick,
                );
                return;
            }
            verse.entities.shift_remove(&uuid);
            nv.list_states.verse_entity_pick_liststate.select(None);
            nv.state = NarraverseState::EntityPick;
            nv.send_footer_notification(String::from("Entity deleted"));
        }
        DeletionTarget::Artifact(uuid) => {
            if !verse.artifacts.contains_key(&uuid) {
                nv.send_error(
                    format!(
                        "Couldn't delete artifact with UUID {}.\nPerhaps it was already deleted.",
                        uuid
                    ),
                    NarraverseState::ArtifactPick,
                );
                return;
            }
            verse.artifacts.shift_remove(&uuid);
            nv.list_states.verse_artifact_pick_liststate.select(None);
            nv.state = NarraverseState::ArtifactPick;
            nv.send_footer_notification(String::from("Artifact deleted"));
        }
    }
    if let Err(err) = save_verses(&mut nv.verses) {
        nv.send_error(
            format!("Cannot finalize deletion:\n{}", err),
            NarraverseState::VersePick,
        );
    }
}
