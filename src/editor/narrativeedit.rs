use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};
use ratatui_textarea::{TextArea, WrapMode};

use crate::nvcore::{
    narraverse::{Narraverse, NarraverseState},
    save::save_verses,
};

pub struct NarrativeEditor {
    pub currently_editing: usize,
    pub textarea: TextArea<'static>,
}

impl Default for NarrativeEditor {
    fn default() -> Self {
        Self {
            currently_editing: 0,
            textarea: TextArea::default(),
        }
    }
}

impl NarrativeEditor {
    pub fn edit(nv: &Narraverse, index: usize) -> Option<Self> {
        let verse = nv.verses.verses.get(nv.verses.verse_index)?;
        let narrative = verse.narratives.get(index)?;
        let mut textarea = TextArea::from(narrative.narrative.lines());
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        Some(Self {
            currently_editing: index,
            textarea,
        })
    }
}

pub fn render_narrative_editor(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        nv.send_error(String::from("No Verse at this index."), NarraverseState::VersePick);
        return;
    };
    let Some(narrative) = verse.narratives.get(nv.narrative_editor.currently_editing) else {
        nv.send_error(String::from("No narrative at this index."), NarraverseState::NarrativePick);
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
    frame.render_widget(&nv.narrative_editor.textarea, editor_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => nv.state = NarraverseState::NarrativePick,
        (KeyCode::Char('w'), KeyModifiers::ALT) => save_narrative(nv),
        _ => {
            nv.narrative_editor.textarea.input(*key);
        }
    }
}

fn save_narrative(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get_mut(nv.verses.verse_index) else {
        return;
    };
    let Some(narrative) = verse
        .narratives
        .get_mut(nv.narrative_editor.currently_editing)
    else {
        return;
    };
    narrative.narrative = nv.narrative_editor.textarea.lines().join("\n");
    if let Err(err) = save_verses(&nv.verses) {
        nv.send_error(format!("Cannot save the narrative:\n{}", err), NarraverseState::NarrativeEdit);
    } else {
        nv.send_footer_notification(String::from("Narrative saved"));
    }
}
