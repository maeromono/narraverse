use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use indexmap::IndexMap;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::Paragraph,
};
use ratatui_textarea::{CursorMove, TextArea, WrapMode};
use uuid::Uuid;

use crate::nvcore::{
    narraverse::{Narraverse, NarraverseState},
    save::save_verse,
    verses::{NarraverseVerses, Verse},
};

pub struct NarraverseEditor {
    buffers: IndexMap<Uuid, EditorBuffer>,
    buffer_index: usize,
}

impl Default for NarraverseEditor {
    fn default() -> Self {
        Self {
            buffers: IndexMap::default(),
            buffer_index: 0,
        }
    }
}

struct EditorBuffer {
    target: EditorTarget,
    textarea: TextArea<'static>,
    cached_name: String,
    modified: bool,
    unsaved_warned: bool,
}

impl NarraverseEditor {
    pub fn add_buffer(
        &mut self,
        verses: &NarraverseVerses,
        target: EditorTarget,
        uuid: Uuid,
    ) -> bool {
        if self.buffers.contains_key(&uuid) {
            return true;
        }
        let Some(verse) = verses.verses.get(&verses.verse_uuid) else {
            return false;
        };
        let (mut textarea, cached_name) = match target {
            EditorTarget::Narrative => {
                let Some(narr) = verse.narratives.get(&uuid) else {
                    return false;
                };
                (TextArea::from(narr.narrative.lines()), narr.title.clone())
            }
            EditorTarget::Entity => {
                let Some(ent) = verse.entities.get(&uuid) else {
                    return false;
                };
                (TextArea::from(ent.details.lines()), ent.name.clone())
            }
            EditorTarget::Artifact => {
                let Some(art) = verse.artifacts.get(&uuid) else {
                    return false;
                };
                (TextArea::from(art.details.lines()), art.name.clone())
            }
        };
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        textarea.set_line_number_style(Style::default().fg(Color::Gray));
        self.buffers.insert(
            uuid,
            EditorBuffer {
                target,
                textarea,
                cached_name,
                modified: false,
                unsaved_warned: false,
            },
        );
        true
    }
}

pub enum EditorTarget {
    Narrative,
    Entity,
    Artifact,
}

impl Display for EditorTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EditorTarget::Narrative => "narrative",
                EditorTarget::Entity => "entity",
                EditorTarget::Artifact => "artifact",
            }
        )
    }
}

pub fn render_editor(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(buffer) = nv.editor.buffers.get_index(nv.editor.buffer_index) else {
        nv.send_error(
            String::from("Invalid buffer"),
            NarraverseState::VerseAreaPick,
        );
        return;
    };
    let editor_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);
    frame.render_widget(
        Paragraph::new(format!("({}) {}", buffer.1.target, buffer.1.cached_name)).bold(),
        editor_layout[0],
    );
    frame.render_widget(&buffer.1.textarea, editor_layout[1]);
    if buffer.1.unsaved_warned {
        frame.render_widget(
            Paragraph::new("Unsaved changes, press Esc again to discard")
                .centered()
                .yellow()
                .bold()
                .reversed(),
            editor_layout[2],
        );
    }
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    let Some(buffer) = nv.editor.buffers.get_index_mut(nv.editor.buffer_index) else {
        nv.send_error(
            String::from("Invalid buffer"),
            NarraverseState::VerseAreaPick,
        );
        return;
    };
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => {
            // if buffer.1.modified && !buffer.1.unsaved_warned {
            //     buffer.1.unsaved_warned = true;
            //     return;
            // }
            nv.state = match buffer.1.target {
                EditorTarget::Narrative => NarraverseState::NarrativePick,
                EditorTarget::Entity => NarraverseState::EntityPick,
                EditorTarget::Artifact => NarraverseState::ArtifactPick,
            }
        }
        (KeyCode::PageDown, KeyModifiers::CONTROL) => nv.editor.buffer_index += 1,
        (KeyCode::PageUp, KeyModifiers::CONTROL) => nv.editor.buffer_index -= 1,
        (KeyCode::Home, KeyModifiers::CONTROL) => buffer.1.textarea.move_cursor(CursorMove::Top),
        (KeyCode::End, KeyModifiers::CONTROL) => buffer.1.textarea.move_cursor(CursorMove::Bottom),
        (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
            buffer.1.textarea.paste();
        }
        (KeyCode::Char('y'), KeyModifiers::CONTROL) => {
            buffer.1.textarea.copy();
        }
        (KeyCode::Delete, KeyModifiers::CONTROL) => {
            buffer.1.textarea.delete_next_word();
        }
        (KeyCode::Char('w'), KeyModifiers::ALT) => save_buffer(nv),
        _ => {
            buffer.1.modified |= buffer.1.textarea.input(*key);
            buffer.1.unsaved_warned = false;
        }
    }
}

fn save_buffer(nv: &mut Narraverse) {
    let Some(mut verse) = nv.verses.verses.get_key_value_mut(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    let Some(buffer) = nv.editor.buffers.get_index_mut(nv.editor.buffer_index) else {
        return;
    };
    if !buffer.1.modified {
        return;
    }
    if let Err((message, return_state)) = write_unit(
        &mut verse.1,
        &buffer.1.target,
        buffer.0,
        buffer.1.textarea.lines().join("\n"),
    ) {
        nv.send_error(message, return_state);
        return;
    }
    if let Err(err) = save_verse(&verse.1, format!("verse-{}", verse.0).as_str()) {
        nv.send_error(
            format!("Cannot perform the save:\n{}", err),
            NarraverseState::Editor,
        );
    } else {
        buffer.1.modified = false;
        nv.send_footer_notification(String::from("Saved successfully"));
    }
}

fn write_unit(
    verse: &mut Verse,
    target: &EditorTarget,
    uuid: &Uuid,
    text: String,
) -> Result<(), (String, NarraverseState)> {
    match target {
        EditorTarget::Narrative => verse
            .narratives
            .get_mut(uuid)
            .map(|narr| narr.narrative = text)
            .ok_or_else(|| {
                (
                    format!("No narrative with this UUID ({}) exists.", uuid),
                    NarraverseState::NarrativePick,
                )
            }),
        EditorTarget::Entity => verse
            .entities
            .get_mut(uuid)
            .map(|ent| ent.details = text)
            .ok_or_else(|| {
                (
                    format!("No entity with this UUID ({}) exists.", uuid),
                    NarraverseState::EntityPick,
                )
            }),
        EditorTarget::Artifact => verse
            .artifacts
            .get_mut(uuid)
            .map(|art| art.details = text)
            .ok_or_else(|| {
                (
                    format!("No artifact with this UUID ({}) exists.", uuid),
                    NarraverseState::ArtifactPick,
                )
            }),
    }
}
