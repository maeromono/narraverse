use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::nvcore::{
    decosutils::{center_area, field_layout, fields_responsive},
    narraverse::{Narraverse, NarraverseState},
    save::save_verses,
    textareautils::validate_length,
    verses::{NarraverseVerses, Verse},
};

pub struct VerseEdit {
    mode: VerseEditMode,
    verse_name_textarea: TextArea<'static>,
    verse_name_valid: bool,
}

impl VerseEdit {
    pub fn default() -> Self {
        Self {
            mode: VerseEditMode::New,
            verse_name_textarea: TextArea::default(),
            verse_name_valid: false,
        }
    }

    pub fn edit(verses: &NarraverseVerses, uuid: Uuid) -> Option<Self> {
        let verse = verses.verses.get(&uuid)?;
        Some(Self {
            mode: VerseEditMode::Existing(uuid),
            verse_name_textarea: TextArea::new(vec![verse.name.clone()]),
            verse_name_valid: true,
        })
    }
}

enum VerseEditMode {
    New,
    Existing(Uuid),
}

pub fn render_verse_edit(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let ve_layout = Layout::vertical([vert_constraint; 1]) // bcuz more coming soon
        .split(center_area(area));
    let field_layout = field_layout(direction);
    let name_field_layout = field_layout.split(ve_layout[0]);
    frame.render_widget(Paragraph::new("Verse name"), name_field_layout[0]);
    frame.render_widget(
        &nv.verse_edit_modal.verse_name_textarea,
        name_field_layout[1],
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::VersePick;
            reset_textareas(nv);
        }
        KeyCode::Enter => confirm_edit(nv),
        _ => {
            let textarea = &mut nv.verse_edit_modal.verse_name_textarea;
            if textarea.input(*key) {
                nv.verse_edit_modal.verse_name_valid = validate_length(textarea, 1, 50);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.verse_edit_modal.verse_name_valid {
        return;
    }
    match nv.verse_edit_modal.mode {
        VerseEditMode::New => {
            nv.verses.verses.insert(
                Uuid::now_v7(),
                Verse {
                    name: nv.verse_edit_modal.verse_name_textarea.lines()[0].clone(),
                    ..Default::default()
                },
            );
            nv.state = NarraverseState::VersePick;
        }
        VerseEditMode::Existing(uuid) => {
            let Some(verse) = nv.verses.verses.get_mut(&uuid) else {
                nv.send_error(
                    format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
                    NarraverseState::VersePick,
                );
                return;
            };
            verse.name = nv.verse_edit_modal.verse_name_textarea.lines()[0].clone();
            nv.state = NarraverseState::VersePick;
        }
    }
    if let Err(err) = save_verses(&mut nv.verses) {
        nv.send_error(
            format!("Cannot confirm edit:\n{}", err),
            NarraverseState::VersePick,
        );
    }
    reset_textareas(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.verse_edit_modal.verse_name_textarea = TextArea::default();
}
