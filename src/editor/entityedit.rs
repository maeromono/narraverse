use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle, previous_cycle};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;

use crate::nvcore::{
    decosutils::{center_area, focus_highlight},
    narraverse::{Narraverse, NarraverseState},
    save::save_verses,
    textareautils::validate_length,
    verses::{Entity, EntityType},
};

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

    pub fn edit(nv: &Narraverse, ind: usize) -> Option<Self> {
        let verse = nv.verses.verses.get(nv.verses.verse_index)?;
        let entity = verse.entities.get(ind)?;
        Some(Self {
            mode: EntityEditMode::Existing(ind),
            current_focus: CurrentFocus::Name,
            entity_name_textarea: TextArea::new(vec![entity.name.clone()]),
            entity_name_valid: true,
            entity_type: entity.entity_type.clone(),
        })
    }
}

enum EntityEditMode {
    New,
    Existing(usize),
}

#[derive(Sequence, PartialEq, Eq)]
enum CurrentFocus {
    Name,
    Type,
}

pub fn render_entity_edit(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = if area.width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    };
    let ve_layout = Layout::vertical([vert_constraint; 2]).split(center_area(area));
    let field_layout = Layout::default()
        .direction(direction)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)]);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    let type_field_layout = field_layout.clone().split(ve_layout[1]);
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
        Paragraph::new(format!("< {} >", nv.entity_edit_modal.entity_type)),
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
    let Some(verse) = nv.verses.verses.get_mut(nv.verses.verse_index) else {
        nv.send_error(
            String::from("No Verse at this index."),
            NarraverseState::VersePick,
        );
        return;
    };
    match nv.entity_edit_modal.mode {
        EntityEditMode::New => {
            verse.entities.push(Entity {
                name: nv.entity_edit_modal.entity_name_textarea.lines()[0].clone(),
                entity_type: nv.entity_edit_modal.entity_type.clone(),
            });
            nv.state = NarraverseState::EntityPick;
        }
        EntityEditMode::Existing(index) => {
            let Some(entity) = verse.entities.get_mut(index) else {
                nv.send_error(
                    String::from("No entity at this index."),
                    NarraverseState::EntityPick,
                );
                return;
            };
            entity.name = nv.entity_edit_modal.entity_name_textarea.lines()[0].clone();
            entity.entity_type = nv.entity_edit_modal.entity_type.clone();
            nv.state = NarraverseState::EntityPick;
        }
    }
    if let Err(err) = save_verses(&nv.verses) {
        nv.send_error(
            format!("Entity edited, but won't save:\n{}", err),
            NarraverseState::EntityPick,
        );
    } else {
        nv.send_footer_notification(String::from("Entity edited"));
    }
    reset_textareas(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.entity_edit_modal.entity_name_textarea = TextArea::default();
    nv.entity_edit_modal.entity_type = EntityType::Character;
}
