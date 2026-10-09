use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle, previous_cycle};
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::{nvcore::{
    narraverse::{Narraverse, NarraverseState},
    save::save_verse,
    verses::{Entity, EntityType, NarraverseVerses},
}, utils::{cacheutils::rebuild_ent_list_cache, decosutils::{center_area, field_layout, fields_responsive, focus_highlight}, textareautils::validate_length}};

pub struct EntityEditModal {
    mode: EntityEditMode,
    current_focus: CurrentFocus,
    entity_name_textarea: TextArea<'static>,
    entity_name_valid: bool,
    entity_type: EntityType,
}

impl EntityEditModal {
    pub fn default() -> Self {
        Self {
            mode: EntityEditMode::New,
            current_focus: CurrentFocus::Name,
            entity_name_textarea: TextArea::default(),
            entity_name_valid: false,
            entity_type: EntityType::Character,
        }
    }

    pub fn edit(verses: &NarraverseVerses, uuid: Uuid) -> Option<Self> {
        let verse = verses.verses.get(&verses.verse_uuid)?;
        let entity = verse.entities.get(&uuid)?;
        Some(Self {
            mode: EntityEditMode::Existing(uuid),
            current_focus: CurrentFocus::Name,
            entity_name_textarea: TextArea::new(vec![entity.name.clone()]),
            entity_name_valid: true,
            entity_type: entity.entity_type.clone(),
        })
    }
}

enum EntityEditMode {
    New,
    Existing(Uuid),
}

#[derive(Sequence, PartialEq, Eq)]
enum CurrentFocus {
    Name,
    Type,
}

pub fn render_entity_edit(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let ve_layout = Layout::vertical([vert_constraint; 2]).split(center_area(area));
    let field_layout = field_layout(direction);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    let type_field_layout = field_layout.split(ve_layout[1]);
    let focus = &nv.entity_edit_modal.current_focus;
    frame.render_widget(
        Paragraph::new("Entity name").style(focus_highlight(matches!(focus, CurrentFocus::Name))),
        name_field_layout[0],
    );
    frame.render_widget(
        &nv.entity_edit_modal.entity_name_textarea,
        name_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Entity type").style(focus_highlight(matches!(focus, CurrentFocus::Type))),
        type_field_layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!(
            "< {} >",
            nv.entity_edit_modal.entity_type.full_name()
        )),
        type_field_layout[1],
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::EntityPick;
            reset_textareas(nv);
            return;
        }
        KeyCode::Tab => {
            nv.entity_edit_modal.current_focus = next_cycle(&nv.entity_edit_modal.current_focus);
            return;
        }
        KeyCode::BackTab => {
            nv.entity_edit_modal.current_focus =
                previous_cycle(&nv.entity_edit_modal.current_focus);
            return;
        }
        KeyCode::Enter => {
            confirm_edit(nv);
            return;
        }
        _ => {}
    }
    match nv.entity_edit_modal.current_focus {
        CurrentFocus::Type => match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                nv.entity_edit_modal.entity_type =
                    previous_cycle(&nv.entity_edit_modal.entity_type);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                nv.entity_edit_modal.entity_type = next_cycle(&nv.entity_edit_modal.entity_type);
            }
            _ => {}
        },
        CurrentFocus::Name => {
            let textarea = &mut nv.entity_edit_modal.entity_name_textarea;
            if textarea.input(*key) {
                nv.entity_edit_modal.entity_name_valid = validate_length(textarea, 1, 100);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.entity_edit_modal.entity_name_valid {
        return;
    }
    let Some(verse) = nv.verses.verses.get_key_value_mut(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    match nv.entity_edit_modal.mode {
        EntityEditMode::New => {
            verse.1.entities.insert(
                Uuid::now_v7(),
                Entity {
                    name: nv.entity_edit_modal.entity_name_textarea.lines()[0].clone(),
                    entity_type: nv.entity_edit_modal.entity_type.clone(),
                    details: String::new(),
                },
            );
            nv.state = NarraverseState::EntityPick;
        }
        EntityEditMode::Existing(uuid) => {
            let Some(entity) = verse.1.entities.get_mut(&uuid) else {
                nv.send_error(
                    format!("No entity with this UUID ({}) exists.", uuid),
                    NarraverseState::EntityPick,
                );
                return;
            };
            entity.name = nv.entity_edit_modal.entity_name_textarea.lines()[0].clone();
            entity.entity_type = nv.entity_edit_modal.entity_type.clone();
            nv.state = NarraverseState::EntityPick;
        }
    }
    if let Err(err) = save_verse(&verse.1, format!("verse-{}", verse.0).as_str()) {
        nv.send_error(
            format!("Entity edited, but won't save:\n{}", err),
            NarraverseState::EntityPick,
        );
    } else {
        nv.send_footer_notification(String::from("Entity edited"));
    }
    reset_textareas(nv);
    rebuild_ent_list_cache(&mut nv.cache, &nv.verses);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.entity_edit_modal.entity_name_textarea = TextArea::default();
    nv.entity_edit_modal.entity_type = EntityType::Character;
}
