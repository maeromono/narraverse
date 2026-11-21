use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle, previous_cycle};
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
        verses::{Artifact, ArtifactType, NarraverseVerses},
    },
    utils::{
        cacheutils::rebuild_art_list_cache, decosutils::{center_area, field_layout, fields_responsive, focus_highlight}, nvutils::save_whole_verse, textareautils::validate_length
    },
};

pub struct ArtifactEditModal {
    mode: ArtifactEditMode,
    current_focus: CurrentFocus,
    artifact_name_textarea: TextArea<'static>,
    artifact_name_valid: bool,
    artifact_type: ArtifactType,
}

impl ArtifactEditModal {
    pub fn edit(verses: &NarraverseVerses, uuid: Uuid) -> Option<Self> {
        let verse = verses.verses.get(&verses.verse_uuid)?;
        let entity = verse.artifacts.get(&uuid)?;
        Some(Self {
            mode: ArtifactEditMode::Existing(uuid),
            current_focus: CurrentFocus::Name,
            artifact_name_textarea: TextArea::new(vec![entity.name.clone()]),
            artifact_name_valid: true,
            artifact_type: entity.artifact_type.clone(),
        })
    }
}

impl Default for ArtifactEditModal {
    fn default() -> Self {
        Self {
            mode: ArtifactEditMode::New,
            current_focus: CurrentFocus::Name,
            artifact_name_textarea: TextArea::default(),
            artifact_name_valid: false,
            artifact_type: ArtifactType::Concept,
        }
    }
}

enum ArtifactEditMode {
    New,
    Existing(Uuid),
}

#[derive(Sequence, PartialEq, Eq)]
enum CurrentFocus {
    Name,
    Type,
}

pub fn render_artifact_edit(modal: &ArtifactEditModal, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let ve_layout = Layout::vertical([vert_constraint; 2]).split(center_area(area));
    let field_layout = field_layout(direction);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    let type_field_layout = field_layout.split(ve_layout[1]);
    let focus = &modal.current_focus;
    frame.render_widget(
        Paragraph::new("Artifact name").style(focus_highlight(matches!(focus, CurrentFocus::Name))),
        name_field_layout[0],
    );
    frame.render_widget(&modal.artifact_name_textarea, name_field_layout[1]);
    frame.render_widget(
        Paragraph::new("Artifact type").style(focus_highlight(matches!(focus, CurrentFocus::Type))),
        type_field_layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!("< {} >", modal.artifact_type.name())),
        type_field_layout[1],
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::EntityPick;
            reset_textareas(&mut nv.modals.artifact_edit_modal);
            return;
        }
        KeyCode::Tab => {
            nv.modals.artifact_edit_modal.current_focus =
                next_cycle(&nv.modals.artifact_edit_modal.current_focus);
            return;
        }
        KeyCode::BackTab => {
            nv.modals.artifact_edit_modal.current_focus =
                previous_cycle(&nv.modals.artifact_edit_modal.current_focus);
            return;
        }
        KeyCode::Enter => {
            confirm_edit(nv);
            return;
        }
        _ => {}
    }
    match nv.modals.artifact_edit_modal.current_focus {
        CurrentFocus::Type => match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                nv.modals.artifact_edit_modal.artifact_type =
                    previous_cycle(&nv.modals.artifact_edit_modal.artifact_type);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                nv.modals.artifact_edit_modal.artifact_type =
                    next_cycle(&nv.modals.artifact_edit_modal.artifact_type);
            }
            _ => {}
        },
        CurrentFocus::Name => {
            let textarea = &mut nv.modals.artifact_edit_modal.artifact_name_textarea;
            if textarea.input(*key) {
                nv.modals.artifact_edit_modal.artifact_name_valid =
                    validate_length(textarea, 1, 100);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.modals.artifact_edit_modal.artifact_name_valid {
        return;
    }
    let verse = nv.verses.current_verse_kv_mut();
    match nv.modals.artifact_edit_modal.mode {
        ArtifactEditMode::New => {
            verse.1.artifacts.insert(
                Uuid::now_v7(),
                Artifact {
                    name: nv.modals.artifact_edit_modal.artifact_name_textarea.lines()[0].clone(),
                    artifact_type: nv.modals.artifact_edit_modal.artifact_type.clone(),
                    details: String::new(),
                },
            );
            nv.state = NarraverseState::ArtifactPick;
        }
        ArtifactEditMode::Existing(uuid) => {
            let Some(entity) = verse.1.artifacts.get_mut(&uuid) else {
                nv.send_error(
                    format!("Artifact with this UUID ({}) doesn't exist.", uuid),
                    NarraverseState::VersePick,
                );
                return;
            };
            entity.name = nv.modals.artifact_edit_modal.artifact_name_textarea.lines()[0].clone();
            entity.artifact_type = nv.modals.artifact_edit_modal.artifact_type.clone();
            nv.state = NarraverseState::ArtifactPick;
        }
    }
    if let Err(err) = save_whole_verse(verse) {
        nv.send_error(
            format!("Artifact edited, but won't save:\n{}", err),
            NarraverseState::ArtifactPick,
        );
    } else {
        nv.send_footer_notification(String::from("Artifact edited"));
    }
    reset_textareas(&mut nv.modals.artifact_edit_modal);
    rebuild_art_list_cache(&mut nv.cache, &nv.verses);
}

fn reset_textareas(modal: &mut ArtifactEditModal) {
    modal.artifact_name_textarea = TextArea::default();
    modal.artifact_type = ArtifactType::Concept;
}
