use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{ListState, Paragraph},
};

use crate::{
    editor::{
        artifacteditmodal::{self, ArtifactEditModal, render_artifact_edit},
        artifactpick::{self, render_artifact_pick},
        bindhelp::{self, BindhelpMode, render_bindhelp},
        configeditmodal::{self, ConfigEditModal, render_config_edit_modal},
        delete::{self, DeletionTarget, render_delete_screen},
        entityedit::{self, EntityEditModal, render_entity_edit},
        entitypick::{self, render_entity_pick},
        narrativeeditmodal::{self, NarrativeEditModal, render_narrative_edit_modal},
        narrativepick::{self, render_narrative_pick},
        nveditor::{self, NarraverseEditor, render_editor},
        verseareapick::{self, render_area_pick},
        verseedit::{self, VerseEdit, render_verse_edit},
        versepick::{self, render_worldpick},
    },
    nvcore::{
        config::NarraverseConfig,
        error::{self, render_error_screen},
        narraversemain::{render_footer, render_header},
        verses::NarraverseVerses,
    },
};

pub struct Narraverse {
    core: NarraverseCore,
    pub verses: NarraverseVerses,
    pub cached_verse_name: String,
    pub list_states: NarraverseListStates,
    pub modals: NarraverseModals,
    pub editor: NarraverseEditor,
    pub deletion_target: DeletionTarget,
    pub errors: (String, NarraverseState),
    pub footer_notification: Option<String>,
    pub bindhelp_mode: BindhelpMode,
    pub cache: NarraverseCaches,
    pub error_scroll: u16, // temporary
}

struct NarraverseCore {
    config: NarraverseConfig,
    state: NarraverseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NarraverseState {
    VersePick,
    VerseEdit,
    VerseAreaPick,
    NarrativePick,
    NarrativeModal,
    EntityPick,
    EntityModal,
    ArtifactPick,
    ArtifactModal,
    ConfigModal,
    Editor,
    VerseDelete,
    BindHelp,
    Error,
    Quit,
}

impl Narraverse {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while *self.get_state() != NarraverseState::Quit {
            term.draw(|frame| self.render(frame))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keys(&key),
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        if area.width < 50 || area.height < 25 {
            let ttst = Paragraph::new(format!(
                "Terminal too small\nRequired 50x25\nCurrent {}x{}",
                area.width, area.height
            ))
            .red()
            .bold();
            frame.render_widget(ttst, area);
            return;
        }
        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area.centered_horizontally(Constraint::Max(100)));
        match self.get_state() {
            NarraverseState::VersePick => render_worldpick(self, frame, main_layout[1]),
            NarraverseState::VerseEdit => {
                render_verse_edit(&self.modals.verse_edit_modal, frame, main_layout[1])
            }
            NarraverseState::VerseAreaPick => render_area_pick(self, frame, main_layout[1]),
            NarraverseState::NarrativePick => render_narrative_pick(self, frame, main_layout[1]),
            NarraverseState::NarrativeModal => render_narrative_edit_modal(
                &self.modals.narrative_edit_modal,
                frame,
                main_layout[1],
            ),
            NarraverseState::EntityPick => render_entity_pick(self, frame, main_layout[1]),
            NarraverseState::EntityModal => {
                render_entity_edit(&self.modals.entity_edit_modal, frame, main_layout[1])
            }
            NarraverseState::ArtifactPick => render_artifact_pick(self, frame, main_layout[1]),
            NarraverseState::ArtifactModal => {
                render_artifact_edit(&self.modals.artifact_edit_modal, frame, main_layout[1])
            }
            NarraverseState::ConfigModal => {
                render_config_edit_modal(&self.modals.config_edit_modal, frame, main_layout[1])
            }
            NarraverseState::Editor => render_editor(self, frame, main_layout[1]),
            NarraverseState::VerseDelete => render_delete_screen(self, frame, main_layout[1]),
            NarraverseState::BindHelp => render_bindhelp(self, frame, main_layout[1]),
            NarraverseState::Error => render_error_screen(self, frame, main_layout[1]),
            _ => unreachable!(),
        }
        render_header(self, frame, main_layout[0]);
        if let Some(message) = self.footer_notification.clone() {
            frame.render_widget(
                Paragraph::new(message).centered().bold().reversed(),
                main_layout[2],
            );
            self.footer_notification = None;
        } else {
            render_footer(self, frame, main_layout[2]);
        }
    }

    fn handle_keys(&mut self, key: &KeyEvent) {
        match self.get_state() {
            NarraverseState::VersePick => versepick::handle_keys(self, key),
            NarraverseState::VerseEdit => verseedit::handle_keys(self, key),
            NarraverseState::VerseAreaPick => verseareapick::handle_keys(self, key),
            NarraverseState::NarrativePick => narrativepick::handle_keys(self, key),
            NarraverseState::NarrativeModal => narrativeeditmodal::handle_keys(self, key),
            NarraverseState::EntityPick => entitypick::handle_keys(self, key),
            NarraverseState::EntityModal => entityedit::handle_keys(self, key),
            NarraverseState::ArtifactPick => artifactpick::handle_keys(self, key),
            NarraverseState::ArtifactModal => artifacteditmodal::handle_keys(self, key),
            NarraverseState::ConfigModal => configeditmodal::handle_keys(self, key),
            NarraverseState::Editor => nveditor::handle_keys(self, key),
            NarraverseState::VerseDelete => delete::handle_keys(self, key),
            NarraverseState::BindHelp => bindhelp::handle_keys(self, key),
            NarraverseState::Error => error::handle_keys(self, key),
            _ => {}
        }
    }

    pub fn send_error(&mut self, message: String, return_state: NarraverseState) {
        self.error_scroll = 0;
        self.errors.0 = message;
        self.errors.1 = return_state;
        self.set_state(NarraverseState::Error);
    }

    pub fn send_footer_notification(&mut self, message: String) {
        self.footer_notification = Some(message);
    }

    pub fn get_config(&self) -> &NarraverseConfig {
        &self.core.config
    }

    pub fn config_mut(&mut self) -> &mut NarraverseConfig {
        &mut self.core.config
    }

    pub fn replace_config(&mut self, config: NarraverseConfig) {
        self.core.config = config;
    }

    pub fn get_state(&self) -> &NarraverseState {
        &self.core.state
    }

    pub fn set_state(&mut self, next: NarraverseState) {
        self.core.state = next;
    }
}

impl Default for Narraverse {
    fn default() -> Self {
        Self {
            core: NarraverseCore::default(),
            verses: NarraverseVerses::default(),
            cached_verse_name: String::from("maero.surge.sh"),
            list_states: NarraverseListStates::default(),
            modals: NarraverseModals::default(),
            editor: NarraverseEditor::default(),
            deletion_target: DeletionTarget::Verse,
            errors: (String::new(), NarraverseState::VersePick),
            footer_notification: None,
            bindhelp_mode: BindhelpMode::Editor,
            cache: NarraverseCaches::default(),
            error_scroll: 0,
        }
    }
}

impl Default for NarraverseCore {
    fn default() -> Self {
        Self {
            config: NarraverseConfig::default(),
            state: NarraverseState::VersePick,
        }
    }
}

#[derive(Default)]
pub struct NarraverseModals {
    pub verse_edit_modal: VerseEdit,
    pub narrative_edit_modal: NarrativeEditModal,
    pub entity_edit_modal: EntityEditModal,
    pub artifact_edit_modal: ArtifactEditModal,
    pub config_edit_modal: ConfigEditModal,
}

pub struct NarraverseListStates {
    pub verse_pick_liststate: ListState,
    pub verse_area_pick_liststate: ListState,
    pub verse_narrative_pick_liststate: ListState,
    pub verse_entity_pick_liststate: ListState,
    pub verse_artifact_pick_liststate: ListState,
    pub bind_help_liststate: ListState,
}

impl Default for NarraverseListStates {
    fn default() -> Self {
        Self {
            verse_pick_liststate: ListState::default(),
            verse_area_pick_liststate: ListState::default().with_selected(Some(0)),
            verse_narrative_pick_liststate: ListState::default(),
            verse_entity_pick_liststate: ListState::default(),
            verse_artifact_pick_liststate: ListState::default(),
            bind_help_liststate: ListState::default().with_selected(Some(0)),
        }
    }
}

#[derive(Default)]
pub struct NarraverseCaches {
    pub verse_list_cache: Vec<String>,
    pub narrative_list_cache: Vec<String>,
    pub entity_list_cache: Vec<String>,
    pub artifact_list_cache: Vec<String>,
    pub bindhelp_bindings_cache: Vec<String>,
}
