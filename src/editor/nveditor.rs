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
    editorbindhelp::BindhelpMode,
    narraverse::{Narraverse, NarraverseState},
    nvutils::enter_bindhelp,
    save::save_verse,
    textareautils::searchbox_style,
    verses::{NarraverseVerses, Verse},
};

pub struct NarraverseEditor {
    mode: EditorMode,
    buffers: IndexMap<Uuid, EditorBuffer>,
    buffer_index: usize,
    searching_textarea: TextArea<'static>,
}

impl Default for NarraverseEditor {
    fn default() -> Self {
        Self {
            mode: EditorMode::Editing,
            buffers: IndexMap::default(),
            buffer_index: 0,
            searching_textarea: TextArea::default(),
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

    pub fn remove_buffer_uuid(&mut self, uuid: &Uuid) {
        self.buffers.shift_remove(uuid);
        let len = self.buffers.len();
        if self.buffer_index >= len {
            self.buffer_index = len.saturating_sub(1);
        }
    }

    pub fn remove_current_buffer(&mut self) -> bool {
        let index = self.buffer_index;
        let Some((uuid, modified, unsaved_warned)) = self
            .buffers
            .get_index(index)
            .map(|(uuid, buffer)| (*uuid, buffer.modified, buffer.unsaved_warned))
        else {
            return false;
        };
        if modified && !unsaved_warned {
            let Some((_, buffer)) = self.buffers.get_index_mut(index) else {
                return false;
            };
            buffer.unsaved_warned = true;
            return false;
        }
        self.remove_buffer_uuid(&uuid);
        true
    }

    pub fn purge_all_buffers(&mut self) {
        self.buffers = IndexMap::default();
        self.buffer_index = 0;
    }

    fn search_height_open(&self) -> (u16, bool) {
        (
            if self.mode == EditorMode::Searching {
                1
            } else {
                0
            },
            self.mode == EditorMode::Searching,
        )
    }
}

#[derive(PartialEq, Eq)]
enum EditorMode {
    Editing,
    Searching,
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
    let Some((_, buffer)) = nv.editor.buffers.get_index(nv.editor.buffer_index) else {
        nv.send_error(
            String::from("Invalid buffer"),
            NarraverseState::VerseAreaPick,
        );
        return;
    };
    let (search_height, search_open) = nv.editor.search_height_open();
    let editor_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(search_height),
    ])
    .split(area);
    frame.render_widget(
        Paragraph::new(format!(
            "[{}/{}] ({}) {}",
            nv.editor.buffer_index + 1,
            nv.editor.buffers.len(),
            buffer.target,
            buffer.cached_name
        ))
        .centered()
        .bold(),
        editor_layout[0],
    );
    frame.render_widget(&buffer.textarea, editor_layout[1]);
    if search_open {
        frame.render_widget(&nv.editor.searching_textarea, editor_layout[3]);
    }
    if buffer.unsaved_warned {
        frame.render_widget(
            Paragraph::new("Unsaved changes, press Alt+x again to discard")
                .centered()
                .yellow()
                .bold()
                .reversed(),
            editor_layout[2],
        );
    }
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match nv.editor.mode {
        EditorMode::Editing => handle_editing_keys(nv, key),
        EditorMode::Searching => handle_searching_keys(nv, key),
    }
}

pub fn handle_searching_keys(nv: &mut Narraverse, key: &KeyEvent) {
    let Some((_, buffer)) = nv.editor.buffers.get_index_mut(nv.editor.buffer_index) else {
        nv.send_error(
            String::from("Invalid buffer"),
            NarraverseState::VerseAreaPick,
        );
        return;
    };
    match key.code {
        KeyCode::Esc => {
            nv.editor.mode = EditorMode::Editing;
            nv.editor.searching_textarea.clear();
            buffer.textarea.set_search_pattern("").unwrap();
        }
        KeyCode::Down => {
            if !buffer.textarea.search_forward(false) {
                nv.send_footer_notification(String::from("No pattern found"));
            }
        }
        KeyCode::Up => {
            if !buffer.textarea.search_back(false) {
                nv.send_footer_notification(String::from("No pattern found"));
            }
        }
        KeyCode::Enter => {
            let found = buffer.textarea.search_forward(true);
            nv.editor.searching_textarea.clear();
            buffer.textarea.set_search_pattern("").unwrap();
            if !found {
                nv.send_footer_notification(String::from("No pattern found"));
            }
            nv.editor.mode = EditorMode::Editing;
        }
        _ => {
            let textarea = &mut nv.editor.searching_textarea;
            if textarea.input(*key) {
                searchbox_style(
                    textarea,
                    buffer
                        .textarea
                        .set_search_pattern(textarea.lines()[0].as_str()),
                );
            }
        }
    }
}

pub fn handle_editing_keys(nv: &mut Narraverse, key: &KeyEvent) {
    if (key.code, key.modifiers) == (KeyCode::Char('x'), KeyModifiers::ALT) {
        if nv.editor.remove_current_buffer() && nv.editor.buffers.is_empty() {
            nv.state = NarraverseState::VerseAreaPick;
        }
        return;
    }
    let Some((_, buffer)) = nv.editor.buffers.get_index_mut(nv.editor.buffer_index) else {
        nv.send_error(
            String::from("Invalid buffer"),
            NarraverseState::VerseAreaPick,
        );
        return;
    };
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => {
            nv.state = match buffer.target {
                EditorTarget::Narrative => NarraverseState::NarrativePick,
                EditorTarget::Entity => NarraverseState::EntityPick,
                EditorTarget::Artifact => NarraverseState::ArtifactPick,
            }
        }
        (KeyCode::F(1), KeyModifiers::NONE) => enter_bindhelp(nv, BindhelpMode::Editor),
        (KeyCode::PageDown, KeyModifiers::CONTROL) => {
            let len = nv.editor.buffers.len();
            if len == 0 {
                return;
            }
            nv.editor.buffer_index = (nv.editor.buffer_index + 1) % len;
        }
        (KeyCode::PageUp, KeyModifiers::CONTROL) => {
            let len = nv.editor.buffers.len();
            if len == 0 {
                return;
            }
            nv.editor.buffer_index = (nv.editor.buffer_index + len - 1) % len;
        }
        (KeyCode::Home, KeyModifiers::CONTROL) => buffer.textarea.move_cursor(CursorMove::Top),
        (KeyCode::End, KeyModifiers::CONTROL) => buffer.textarea.move_cursor(CursorMove::Bottom),
        // vv these specific out of the box binds from the crate don't work for some reason so i
        // had to bind them again
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            buffer.textarea.copy();
        }
        (KeyCode::Char('y'), KeyModifiers::CONTROL) => {
            buffer.textarea.paste();
        }
        (KeyCode::Delete, KeyModifiers::CONTROL) => {
            buffer.textarea.delete_next_word();
        }
        (KeyCode::Char('w'), KeyModifiers::ALT) => save_buffer(nv),
        (KeyCode::Char('f'), KeyModifiers::ALT) => nv.editor.mode = EditorMode::Searching,
        _ => {
            buffer.modified |= buffer.textarea.input(*key);
            buffer.unsaved_warned = false;
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
