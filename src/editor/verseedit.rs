use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::{nvcore::{
    narraverse::{Narraverse, NarraverseState},
    save::{save_verse, save_verses},
    verses::{NarraverseVerses, Verse},
}, utils::{cacheutils::rebuild_verse_list_cache, decosutils::{center_area, field_layout, fields_responsive}, textareautils::validate_length}};

pub struct VerseEdit {
    mode: VerseEditMode,
    verse_name_textarea: TextArea<'static>,
    verse_name_valid: bool,
}

impl VerseEdit {
    pub fn edit(verses: &NarraverseVerses, uuid: Uuid) -> Option<Self> {
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
    frame.render_widget(
        &modal.verse_name_textarea,
        name_field_layout[1],
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::VersePick;
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
    if !nv.modals.verse_edit_modal.verse_name_valid {
        return;
    }
    match nv.modals.verse_edit_modal.mode {
        VerseEditMode::New => {
            let new_verse_uuid = Uuid::now_v7();
            nv.verses.verses.insert(
                new_verse_uuid,
                Verse {
                    name: nv.modals.verse_edit_modal.verse_name_textarea.lines()[0].clone(),
                    ..Default::default()
                },
            );
            if let Err(err) = save_verses(&mut nv.verses) {
                nv.send_error(
                    format!("Cannot save the newly created Verse:\n{}", err),
                    NarraverseState::VersePick,
                );
            }
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
            verse.name = nv.modals.verse_edit_modal.verse_name_textarea.lines()[0].clone();
            nv.state = NarraverseState::VersePick;
            if let Err(err) = save_verse(verse, format!("verse-{}", uuid).as_str()) {
                nv.send_error(
                    format!("Cannot save the Verse:\n{}", err),
                    NarraverseState::VersePick,
                );
            }
        }
    }
    reset_textareas(&mut nv.modals.verse_edit_modal);
    rebuild_verse_list_cache(&mut nv.cache, &nv.verses);
}

fn reset_textareas(modal: &mut VerseEdit) {
    modal.verse_name_textarea = TextArea::default();
}
