use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle, previous_cycle};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::nvcore::{
    cacheutils::rebuild_art_list_cache,
    decosutils::{center_area, field_layout, fields_responsive, focus_highlight},
    narraverse::{Narraverse, NarraverseState},
    save::save_verse,
    textareautils::validate_length,
    verses::{Artifact, ArtifactType, NarraverseVerses},
};

pub struct ArtifactEditModal {
    mode: ArtifactEditMode,
    current_focus: CurrentFocus,
    artifact_name_textarea: TextArea<'static>,
    artifact_name_valid: bool,
    artifact_type: ArtifactType,
}

impl ArtifactEditModal {
    pub fn default() -> Self {
        Self {
            mode: ArtifactEditMode::New,
            current_focus: CurrentFocus::Name,
            artifact_name_textarea: TextArea::default(),
            artifact_name_valid: false,
            artifact_type: ArtifactType::Concept,
        }
    }

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

enum ArtifactEditMode {
    New,
    Existing(Uuid),
}

#[derive(Sequence, PartialEq, Eq)]
enum CurrentFocus {
    Name,
    Type,
}

pub fn render_artifact_edit(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = fields_responsive(area.width);
    let ve_layout = Layout::vertical([vert_constraint; 2]).split(center_area(area));
    let field_layout = field_layout(direction);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    let type_field_layout = field_layout.split(ve_layout[1]);
    let focus = &nv.artifact_edit_modal.current_focus;
    frame.render_widget(
        Paragraph::new("Artifact name").style(focus_highlight(matches!(focus, CurrentFocus::Name))),
        name_field_layout[0],
    );
    frame.render_widget(
        &nv.artifact_edit_modal.artifact_name_textarea,
        name_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Artifact type").style(focus_highlight(matches!(focus, CurrentFocus::Type))),
        type_field_layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!(
            "< {} >",
            nv.artifact_edit_modal.artifact_type.name()
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
            nv.artifact_edit_modal.current_focus =
                next_cycle(&nv.artifact_edit_modal.current_focus);
            return;
        }
        KeyCode::BackTab => {
            nv.artifact_edit_modal.current_focus =
                previous_cycle(&nv.artifact_edit_modal.current_focus);
            return;
        }
        KeyCode::Enter => {
            confirm_edit(nv);
            return;
        }
        _ => {}
    }
    match nv.artifact_edit_modal.current_focus {
        CurrentFocus::Type => match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                nv.artifact_edit_modal.artifact_type =
                    previous_cycle(&nv.artifact_edit_modal.artifact_type);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                nv.artifact_edit_modal.artifact_type =
                    next_cycle(&nv.artifact_edit_modal.artifact_type);
            }
            _ => {}
        },
        CurrentFocus::Name => {
            let textarea = &mut nv.artifact_edit_modal.artifact_name_textarea;
            if textarea.input(*key) {
                nv.artifact_edit_modal.artifact_name_valid = validate_length(textarea, 1, 100);
            }
        }
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.artifact_edit_modal.artifact_name_valid {
        return;
    }
    let Some(verse) = nv.verses.verses.get_key_value_mut(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!(
                "Verse with this UUID ({}) doesn't exist.",
                nv.verses.verse_uuid
            ),
            NarraverseState::VersePick,
        );
        return;
    };
    match nv.artifact_edit_modal.mode {
        ArtifactEditMode::New => {
            verse.1.artifacts.insert(
                Uuid::now_v7(),
                Artifact {
                    name: nv.artifact_edit_modal.artifact_name_textarea.lines()[0].clone(),
                    artifact_type: nv.artifact_edit_modal.artifact_type.clone(),
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
            entity.name = nv.artifact_edit_modal.artifact_name_textarea.lines()[0].clone();
            entity.artifact_type = nv.artifact_edit_modal.artifact_type.clone();
            nv.state = NarraverseState::ArtifactPick;
        }
    }
    if let Err(err) = save_verse(&verse.1, format!("verse-{}", verse.0).as_str()) {
        nv.send_error(
            format!("Artifact edited, but won't save:\n{}", err),
            NarraverseState::ArtifactPick,
        );
    } else {
        nv.send_footer_notification(String::from("Artifact edited"));
    }
    reset_textareas(nv);
    rebuild_art_list_cache(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.artifact_edit_modal.artifact_name_textarea = TextArea::default();
    nv.artifact_edit_modal.artifact_type = ArtifactType::Concept;
}
