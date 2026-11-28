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
        versemanifest::{Artifact, ArtifactType, VerseManifest},
    },
    utils::{
        decosutils::{center_area, field_layout, fields_responsive, focus_highlight},
        nvutils::save_current_verse,
        textareautils::validate_length,
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
    pub fn edit(verses: &VerseManifest, uuid: Uuid) -> Option<Self> {
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

#[derive(Clone, Copy)]
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
            nv.set_state(NarraverseState::ArtifactPick);
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
    let modal = &nv.modals.artifact_edit_modal;
    if !modal.artifact_name_valid {
        return;
    }
    let name = modal.artifact_name_textarea.lines()[0].clone();
    let artifact_type = modal.artifact_type.clone();
    let mode = modal.mode;
    let verse = nv.verse_manifest_mut().current_verse_kv_mut();
    match mode {
        ArtifactEditMode::New => {
            verse.1.artifacts.insert(
                Uuid::now_v7(),
                Artifact {
                    name,
                    artifact_type,
                    details: String::new(),
                },
            );
        }
        ArtifactEditMode::Existing(uuid) => {
            let Some(artifact) = verse.1.artifacts.get_mut(&uuid) else {
                nv.send_error(format!("Artifact with this UUID ({}) doesn't exist.", uuid));
                return;
            };
            artifact.name = name;
            artifact.artifact_type = artifact_type;
        }
    }
    if let Err(err) = save_current_verse(nv.verse_manifest_mut()) {
        nv.send_error(format!("Artifact edited, but won't save:\n{}", err));
    } else {
        nv.set_state(NarraverseState::ArtifactPick);
        nv.send_footer_notification(String::from("Artifact edited"));
    }
    reset_textareas(&mut nv.modals.artifact_edit_modal);
}

fn reset_textareas(modal: &mut ArtifactEditModal) {
    modal.artifact_name_textarea = TextArea::default();
    modal.artifact_type = ArtifactType::Concept;
}
