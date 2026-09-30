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
};

pub struct NarraverseEditor {
    pub mode: EditorMode,
    pub editing_uuid: Uuid,
    pub editor_textarea: TextArea<'static>,
}

impl Default for NarraverseEditor {
    fn default() -> Self {
        Self {
            mode: EditorMode::Narrative,
            editing_uuid: Uuid::default(),
            editor_textarea: TextArea::default(),
        }
    }
}

impl NarraverseEditor {
    pub fn edit(nv: &Narraverse, mode: EditorMode, uuid: Uuid) -> Option<Self> {
        let verse = nv.verses.verses.get(nv.verses.verse_index)?;
        let narrative = verse.narratives.get(&uuid)?;
        let mut textarea = TextArea::from(narrative.narrative.lines());
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        Some(Self {
            mode,
            editing_uuid: uuid,
            editor_textarea: textarea,
        })
    }
}

pub enum EditorMode {
    Narrative,
    Entity,
}

pub fn render_editor(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    let Some(narrative) = verse.narratives.get(&nv.editor.editing_uuid) else {
        nv.send_error(
            String::from("No narrative at this index."),
            NarraverseState::NarrativePick,
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
        Paragraph::new(format!("Editing: {}", narrative.title)).bold(),
        editor_layout[0],
    );
    frame.render_widget(&nv.editor.editor_textarea, editor_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => nv.state = NarraverseState::NarrativePick,
        (KeyCode::Char('w'), KeyModifiers::ALT) => save_narrative(nv),
        _ => {
            nv.editor.editor_textarea.input(*key);
        }
    }
}

fn save_narrative(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get_mut(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    let Some(narrative) = verse
        .narratives
        .get_mut(&nv.editor.editing_uuid)
    else {
        nv.send_error(
            String::from("No narrative at this index."),
            NarraverseState::NarrativePick,
        );
        return;
    };
    narrative.narrative = nv.editor.editor_textarea.lines().join("\n");
    if let Err(err) = save_verse(&verse, format!("verse-{}", verse.uuid).as_str()) {
        nv.send_error(
            format!("Cannot save the narrative:\n{}", err),
            NarraverseState::NarrativeEdit,
        );
    } else {
        nv.send_footer_notification(String::from("Narrative saved"));
    }
}
