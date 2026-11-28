use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::{
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        save::{save_verse, save_verses},
        versemanifest::{Verse, VerseManifest},
    },
    utils::{
        decosutils::{center_area, field_layout, fields_responsive},
        textareautils::validate_length,
    },
};

pub struct VerseEdit {
    mode: VerseEditMode,
    verse_name_textarea: TextArea<'static>,
    verse_name_valid: bool,
}

impl VerseEdit {
    pub fn edit(verses: &VerseManifest, uuid: Uuid) -> Option<Self> {
        let verse = verses.verses.get(&uuid)?;
        Some(Self {
            mode: VerseEditMode::Existing(uuid),
            verse_name_textarea: TextArea::new(vec![verse.name.clone()]),
            verse_name_valid: true,
        })
    }
}

impl Default for VerseEdit {
    fn default() -> Self {
        Self {
            mode: VerseEditMode::New,
            verse_name_textarea: TextArea::default(),
            verse_name_valid: false,
        }
    }
}

enum VerseEditMode {
    New,
    Existing(Uuid),
}

pub fn render_verse_edit(modal: &VerseEdit, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let ve_layout = Layout::vertical([vert_constraint; 1]) // bcuz more coming soon
        .split(center_area(area));
    let field_layout = field_layout(direction);
    let name_field_layout = field_layout.split(ve_layout[0]);
    frame.render_widget(Paragraph::new("Verse name"), name_field_layout[0]);
    frame.render_widget(&modal.verse_name_textarea, name_field_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.set_state(NarraverseState::VersePick);
            reset_textareas(&mut nv.modals.verse_edit_modal);
        }
        KeyCode::Enter => confirm_edit(nv),
        _ => {
            let textarea = &mut nv.modals.verse_edit_modal.verse_name_textarea;
            if textarea.input(*key) {
                nv.modals.verse_edit_modal.verse_name_valid = validate_length(textarea, 1, 50);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    let modal = &nv.modals.verse_edit_modal;
    if !modal.verse_name_valid {
        return;
    }
    let name = modal.verse_name_textarea.lines()[0].clone();
    let mut saved: bool = false;
    match nv.modals.verse_edit_modal.mode {
        VerseEditMode::New => {
            let new_verse_uuid = Uuid::now_v7();
            nv.verse_manifest_mut().verses.insert(
                new_verse_uuid,
                Verse {
                    name,
                    ..Default::default()
                },
            );
            if let Err(err) = save_verses(nv.verse_manifest_mut()) {
                nv.send_error(
                    format!("Cannot save the newly created Verse:\n{}", err),
                    NarraverseState::VersePick,
                );
            } else {
                saved = true;
            }
        }
        VerseEditMode::Existing(uuid) => {
            let Some(verse) = nv.verse_manifest_mut().verses.get_mut(&uuid) else {
                nv.send_error(
                    format!("No Verse with this UUID ({}) exists.", uuid),
                    NarraverseState::VersePick,
                );
                return;
            };
            verse.name = name;
            if let Err(err) = save_verse((&uuid, verse)) {
                nv.send_error(
                    format!("Cannot save Verse UUID {}:\n{}", uuid, err),
                    NarraverseState::VersePick,
                );
            } else {
                saved = true;
            }
        }
    }
    if saved {
        nv.set_state(NarraverseState::VersePick);
    }
    reset_textareas(&mut nv.modals.verse_edit_modal);
}

fn reset_textareas(modal: &mut VerseEdit) {
    modal.verse_name_textarea = TextArea::default();
}
