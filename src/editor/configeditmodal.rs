use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;

use crate::{
    nvcore::{
        config::{NarraverseConfig, save_config},
        narraverse::{Narraverse, NarraverseState},
    },
    utils::{
        decosutils::{center_area, field_layout, fields_responsive},
        textareautils::validate_length,
    },
};

pub struct ConfigEditModal {
    writtenby_prefill_textarea: TextArea<'static>,
    writtenby_prefill_valid: bool,
}

impl ConfigEditModal {
    pub fn edit(config: &NarraverseConfig) -> Self {
        Self {
            writtenby_prefill_textarea: TextArea::new(vec![config.default_written_by.clone()]),
            writtenby_prefill_valid: true,
        }
    }
}

impl Default for ConfigEditModal {
    fn default() -> Self {
        Self {
            writtenby_prefill_textarea: TextArea::default(),
            writtenby_prefill_valid: false,
        }
    }
}

pub fn render_config_edit_modal(modal: &ConfigEditModal, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let cem_layout = Layout::vertical([vert_constraint; 1]).split(center_area(area));
    let field_layout = field_layout(direction);
    let title_field_layout = field_layout.split(cem_layout[0]);
    frame.render_widget(Paragraph::new("'Written by' name"), title_field_layout[0]);
    frame.render_widget(&modal.writtenby_prefill_textarea, title_field_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.set_state(NarraverseState::VersePick);
            reset_textareas(&mut nv.modals.config_edit_modal);
        }
        KeyCode::Enter => confirm_edit(nv),
        _ => {
            let textarea = &mut nv.modals.config_edit_modal.writtenby_prefill_textarea;
            if textarea.input(*key) {
                nv.modals.config_edit_modal.writtenby_prefill_valid =
                    validate_length(textarea, 1, 100);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.modals.config_edit_modal.writtenby_prefill_valid {
        return;
    }
    nv.config_mut().default_written_by = nv
        .modals
        .config_edit_modal
        .writtenby_prefill_textarea
        .lines()[0]
        .clone();
    if let Err(err) = save_config(nv.get_config()) {
        nv.send_error(
            format!("Config edited, but won't save:\n{}", err),
            NarraverseState::VersePick,
        );
    } else {
        nv.send_footer_notification(String::from("Config edited"));
        nv.set_state(NarraverseState::VersePick);
    }
    reset_textareas(&mut nv.modals.config_edit_modal);
}

fn reset_textareas(modal: &mut ConfigEditModal) {
    modal.writtenby_prefill_textarea = TextArea::default();
}
