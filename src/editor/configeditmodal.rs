use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;

use crate::nvcore::{
    config::{NarraverseConfig, save_config},
    decosutils::{center_area, field_layout, fields_responsive},
    narraverse::{Narraverse, NarraverseState},
    textareautils::validate_length,
};

pub struct ConfigEditModal {
    writtenby_prefill_textarea: TextArea<'static>,
    writtenby_prefill_valid: bool,
}

impl ConfigEditModal {
    pub fn default() -> Self {
        Self {
            writtenby_prefill_textarea: TextArea::default(),
            writtenby_prefill_valid: false,
        }
    }

    pub fn edit(config: &NarraverseConfig) -> Self {
        Self {
            writtenby_prefill_textarea: TextArea::new(vec![config.default_written_by.clone()]),
            writtenby_prefill_valid: true,
        }
    }
}

pub fn render_config_edit_modal(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let cem_layout = Layout::vertical([vert_constraint; 1]).split(center_area(area));
    let field_layout = field_layout(direction);
    let title_field_layout = field_layout.split(cem_layout[0]);
    frame.render_widget(Paragraph::new("'Written by' name"), title_field_layout[0]);
    frame.render_widget(
        &nv.config_edit_modal.writtenby_prefill_textarea,
        title_field_layout[1],
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
            let textarea = &mut nv.config_edit_modal.writtenby_prefill_textarea;
            if textarea.input(*key) {
                nv.config_edit_modal.writtenby_prefill_valid = validate_length(textarea, 1, 100);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.config_edit_modal.writtenby_prefill_valid {
        return;
    }
    nv.config.default_written_by =
        nv.config_edit_modal.writtenby_prefill_textarea.lines()[0].clone();
    if let Err(err) = save_config(&nv.config) {
        nv.send_error(
            format!("Config edited, but won't save:\n{}", err),
            NarraverseState::VersePick,
        );
    } else {
        nv.send_footer_notification(String::from("Config edited"));
        nv.state = NarraverseState::VersePick;
    }
    reset_textareas(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.config_edit_modal.writtenby_prefill_textarea = TextArea::default();
}
