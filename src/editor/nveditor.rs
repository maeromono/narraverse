use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};
use ratatui_textarea::{TextArea, WrapMode};
use uuid::Uuid;

use crate::nvcore::{
    narraverse::{Narraverse, NarraverseState},
    save::save_verse,
    verses::NarraverseVerses,
};

pub struct NarraverseEditor {
    pub mode: EditorMode,
    pub editing_uuid: Uuid,
    pub editor_textarea: TextArea<'static>,
    pub cached_name: String,
}

impl Default for NarraverseEditor {
    fn default() -> Self {
        Self {
            mode: EditorMode::Narrative,
            editing_uuid: Uuid::default(),
            editor_textarea: TextArea::default(),
            cached_name: String::from("Unknown"),
        }
    }
}

impl NarraverseEditor {
    pub fn edit(verses: &NarraverseVerses, mode: EditorMode, uuid: Uuid) -> Option<Self> {
        let verse = verses.verses.get(&verses.verse_uuid)?;
        let (mut textarea, cached_name) = match mode {
            EditorMode::Narrative => {
                let narr = verse.narratives.get(&uuid)?;
                (TextArea::from(narr.narrative.lines()), narr.title.clone())
            }
            EditorMode::Entity => {
                let ent = verse.entities.get(&uuid)?;
                (TextArea::from(ent.details.lines()), ent.name.clone())
            }
            EditorMode::Artifact => {
                let art = verse.artifacts.get(&uuid)?;
                (TextArea::from(art.details.lines()), art.name.clone())
            }
        };
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        Some(Self {
            mode,
            editing_uuid: uuid,
            editor_textarea: textarea,
            cached_name,
        })
    }
}

pub enum EditorMode {
    Narrative,
    Entity,
    Artifact,
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EditorMode::Narrative => "a narrative",
                EditorMode::Entity => "an entity",
                EditorMode::Artifact => "an artifact",
            }
        )
    }
}

pub fn render_editor(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let editor_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);
    frame.render_widget(
        Paragraph::new(format!(
            "Editing {}: {}",
            nv.editor.mode, nv.editor.cached_name
        ))
        .bold(),
        editor_layout[0],
    );
    frame.render_widget(&nv.editor.editor_textarea, editor_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => {
            nv.state = match nv.editor.mode {
                EditorMode::Narrative => NarraverseState::NarrativePick,
                EditorMode::Entity => NarraverseState::EntityPick,
                EditorMode::Artifact => NarraverseState::ArtifactPick,
            }
        }
        (KeyCode::Char('w'), KeyModifiers::ALT) => save_narrative(nv),
        _ => {
            nv.editor.editor_textarea.input(*key);
        }
    }
}

fn save_narrative(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get_key_value_mut(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    match nv.editor.mode {
        EditorMode::Narrative => {
            let Some(narrative) = verse.1.narratives.get_mut(&nv.editor.editing_uuid) else {
                nv.send_error(
                    format!(
                        "No narrative with this UUID ({}) exists.",
                        nv.editor.editing_uuid
                    ),
                    NarraverseState::NarrativePick,
                );
                return;
            };
            narrative.narrative = nv.editor.editor_textarea.lines().join("\n");
        }
        EditorMode::Entity => {
            let Some(entity) = verse.1.entities.get_mut(&nv.editor.editing_uuid) else {
                nv.send_error(
                    format!(
                        "No entity with this UUID ({}) exists.",
                        nv.editor.editing_uuid
                    ),
                    NarraverseState::EntityPick,
                );
                return;
            };
            entity.details = nv.editor.editor_textarea.lines().join("\n");
        }
        EditorMode::Artifact => {
            let Some(artifact) = verse.1.artifacts.get_mut(&nv.editor.editing_uuid) else {
                nv.send_error(
                    format!(
                        "No artifact with this UUID ({}) exists.",
                        nv.editor.editing_uuid
                    ),
                    NarraverseState::ArtifactPick,
                );
                return;
            };
            artifact.details = nv.editor.editor_textarea.lines().join("\n");
        }
    }
    if let Err(err) = save_verse(&verse.1, format!("verse-{}", verse.0).as_str()) {
        nv.send_error(
            format!("Cannot perform the save:\n{}", err),
            NarraverseState::Editor,
        );
    } else {
        nv.send_footer_notification(String::from("Saved successfully"));
    }
}
