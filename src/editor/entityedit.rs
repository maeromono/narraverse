use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::Sequence;
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::Paragraph};
use ratatui_textarea::TextArea;

use crate::nvcore::{decosutils::center_area, narraverse::{Narraverse, NarraverseState}, save::save_verses, verses::EntityType};

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
            entity_name_textarea: TextArea::new(vec![verse.name.clone()]),
            entity_name_valid: true,
            entity_type: entity.entity_type.clone()
        })
    }
}

enum EntityEditMode {
    New,
    Existing(usize),
}

#[derive(Sequence)]
enum CurrentFocus {
    Name,
    Type
}

pub fn render_entity_edit(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = if area.width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    };
    let ve_layout = Layout::vertical([vert_constraint; 1]) // bcuz more coming soon
        .split(center_area(area));
    let field_layout = Layout::default()
        .direction(direction)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)]);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    frame.render_widget(Paragraph::new("Entity name"), name_field_layout[0]);
    frame.render_widget(&nv.entity_edit_modal.entity_name_textarea, name_field_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::EntityPick;
            reset_textareas(nv);
        }
        KeyCode::Enter => confirm_edit(nv),
        _ => {
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.entity_edit_modal.entity_name_valid {
        return;
    }
    match nv.entity_edit_modal.mode {
        EntityEditMode::New => {
        }
        EntityEditMode::Existing(index) => {
        }
    }
    if let Err(err) = save_verses(&nv.verses) {
        nv.send_error(
            format!("Cannot confirm edit:\n{}", err),
            NarraverseState::VersePick,
        );
    }
    reset_textareas(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.entity_edit_modal.entity_name_textarea = TextArea::default();
}
