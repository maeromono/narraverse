use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};
use ratatui_textarea::{TextArea, WrapMode};

use crate::apcore::{apverse::{ApVerse, ApVerseState}, save::save_verses};

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
    pub fn edit(av: &ApVerse, index: usize) -> Self {
        let Some(narrative) = av
            .verses
            .verses
            .get(av.verses.verse_index)
            .unwrap()
            .narratives
            .get(index)
        else {
            panic!("narrative not found");
        };
        let mut textarea = TextArea::from(narrative.narrative.lines());
        textarea.set_wrap_mode(WrapMode::WordOrGlyph);
        Self {
            currently_editing: index,
            textarea,
        }
    }
}

pub fn render_narrative_editor(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let Some(verse) = av.verses.verses.get(av.verses.verse_index) else {
        frame.render_widget(
            Paragraph::new("No 'Verse at this index.\nHit Esc to pick another one.")
                .red()
                .bold(),
            area,
        );
        return;
    };
    let Some(narrative) = verse.narratives.get(av.narrative_editor.currently_editing) else {
        frame.render_widget(Paragraph::new("Narrative not found").red().bold(), area);
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
    frame.render_widget(&av.narrative_editor.textarea, editor_layout[1]);
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, KeyModifiers::NONE) => av.state = ApVerseState::NarrativePick,
        (KeyCode::Char('w'), KeyModifiers::ALT) => save_narrative(av),
        _ => {
            av.narrative_editor.textarea.input(*key);
        }
    }
}

fn save_narrative(av: &mut ApVerse) {
    let Some(verse) = av.verses.verses.get_mut(av.verses.verse_index) else {
        return;
    };
    let Some(narrative) = verse.narratives.get_mut(av.narrative_editor.currently_editing) else {
        return;
    };
    narrative.narrative = av.narrative_editor.textarea.lines().join("\n");
    save_verses(&av.verses);
}
