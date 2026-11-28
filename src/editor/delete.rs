use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Wrap},
};
use uuid::Uuid;

use crate::{
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        save::save_verses,
    },
    utils::decosutils::center_area,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeletionTarget {
    Verse,
    Narrative(Uuid),
    Entity(Uuid),
    Artifact(Uuid),
}

fn confirmation_message(nv: &Narraverse) -> Result<Paragraph<'static>, String> {
    let manifest = nv.get_verse_manifest();
    if !manifest.current_verse_exists() {
        return Err(String::from("No verse selected."));
    }
    let verse = manifest.current_verse();
    match nv.deletion_target {
        DeletionTarget::Verse => Ok(Paragraph::new(format!(
            "Delete Verse \"{}\"?\nIt'll be removed from the index, but will remain on disk.",
            verse.name
        ))
        .wrap(Wrap { trim: false })
        .yellow()
        .bold()),
        DeletionTarget::Narrative(uuid) => match verse.narratives.get(&uuid) {
            Some(n) => Ok(Paragraph::new(format!(
                "Delete narrative \"{}\"?\nYou cannot revert this.",
                n.title
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()),
            None => Err(format!(
                "Narrative with this UUID ({}) doesn't exist.",
                uuid
            )),
        },
        DeletionTarget::Entity(uuid) => match verse.entities.get(&uuid) {
            Some(e) => Ok(Paragraph::new(format!(
                "Delete entity \"{}\"?\nYou cannot revert this.",
                e.name
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()),
            None => Err(format!("Entity with this UUID ({}) doesn't exist.", uuid)),
        },
        DeletionTarget::Artifact(uuid) => match verse.artifacts.get(&uuid) {
            Some(a) => Ok(Paragraph::new(format!(
                "Delete artifact \"{}\"?\nYou cannot revert this.",
                a.name
            ))
            .wrap(Wrap { trim: false })
            .yellow()
            .bold()),
            None => Err(format!("Artifact with this UUID ({}) doesn't exist.", uuid)),
        },
    }
}

pub fn render_delete_screen(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    match confirmation_message(nv) {
        Ok(message) => frame.render_widget(message, center_area(area)),
        Err(message) => nv.send_error(message),
    }
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            let next = match nv.deletion_target {
                DeletionTarget::Verse => NarraverseState::VersePick,
                DeletionTarget::Narrative(_) => NarraverseState::NarrativePick,
                DeletionTarget::Entity(_) => NarraverseState::EntityPick,
                DeletionTarget::Artifact(_) => NarraverseState::ArtifactPick,
            };
            nv.set_state(next);
        }
        KeyCode::Char('y') => {
            confirm_delete(nv);
        }
        _ => {}
    }
}

fn confirm_delete(nv: &mut Narraverse) {
    let target = nv.deletion_target;
    let validation: std::result::Result<(), String> = {
        let manifest = nv.get_verse_manifest();
        match target {
            DeletionTarget::Verse => Ok(()),
            DeletionTarget::Narrative(uuid) => {
                if manifest.current_verse_exists()
                    && manifest.current_verse().narratives.contains_key(&uuid)
                {
                    Ok(())
                } else {
                    Err(format!(
                        "Couldn't delete narrative with UUID {}.\nPerhaps it was already deleted.",
                        uuid
                    ))
                }
            }
            DeletionTarget::Entity(uuid) => {
                if manifest.current_verse_exists()
                    && manifest.current_verse().entities.contains_key(&uuid)
                {
                    Ok(())
                } else {
                    Err(format!(
                        "Couldn't delete entity with UUID {}.\nPerhaps it was already deleted.",
                        uuid
                    ))
                }
            }
            DeletionTarget::Artifact(uuid) => {
                if manifest.current_verse_exists()
                    && manifest.current_verse().artifacts.contains_key(&uuid)
                {
                    Ok(())
                } else {
                    Err(format!(
                        "Couldn't delete artifact with UUID {}.\nPerhaps it was already deleted.",
                        uuid
                    ))
                }
            }
        }
    };
    if let Err(message) = validation {
        nv.send_error(message);
        return;
    }
    {
        let manifest = nv.verse_manifest_mut();
        match target {
            DeletionTarget::Verse => {
                let uuid = manifest.verse_uuid;
                manifest.verses.shift_remove(&uuid);
                manifest.verse_uuid = Uuid::default();
            }
            DeletionTarget::Narrative(uuid) => {
                manifest.current_verse_mut().narratives.shift_remove(&uuid);
            }
            DeletionTarget::Entity(uuid) => {
                manifest.current_verse_mut().entities.shift_remove(&uuid);
            }
            DeletionTarget::Artifact(uuid) => {
                manifest.current_verse_mut().artifacts.shift_remove(&uuid);
            }
        }
    }
    match target {
        DeletionTarget::Verse => {
            nv.editor.purge_all_buffers();
            nv.list_states.verse_pick_liststate.select(None);
            nv.set_state(NarraverseState::VersePick);
            nv.send_footer_notification(String::from("Verse deleted"));
        }
        DeletionTarget::Narrative(uuid) => {
            nv.list_states.verse_narrative_pick_liststate.select(None);
            nv.editor.remove_buffer_uuid(&uuid);
            nv.set_state(NarraverseState::NarrativePick);
            nv.send_footer_notification(String::from("Narrative deleted"));
        }
        DeletionTarget::Entity(uuid) => {
            nv.list_states.verse_entity_pick_liststate.select(None);
            nv.editor.remove_buffer_uuid(&uuid);
            nv.set_state(NarraverseState::EntityPick);
            nv.send_footer_notification(String::from("Entity deleted"));
        }
        DeletionTarget::Artifact(uuid) => {
            nv.list_states.verse_artifact_pick_liststate.select(None);
            nv.editor.remove_buffer_uuid(&uuid);
            nv.set_state(NarraverseState::ArtifactPick);
            nv.send_footer_notification(String::from("Artifact deleted"));
        }
    }
    if let Err(err) = save_verses(nv.verse_manifest_mut()) {
        nv.send_error(format!("Cannot finalize deletion:\n{}", err));
    }
}
