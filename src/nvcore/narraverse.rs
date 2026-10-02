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
        configeditmodal::{self, ConfigEditModal, render_config_edit_modal},
        entityedit::{self, EntityEditModal, render_entity_edit},
        entitypick::{self, render_entity_pick},
        narrativeeditmodal::{self, NarrativeEditModal, render_narrative_edit_modal},
        nveditor::{self, NarraverseEditor, render_editor},
        verseareapick::{self, render_area_pick},
        versedelete::{self, DeletionTarget, render_delete_screen},
        verseedit::{self, VerseEdit, render_verse_edit},
        versenarrativepick::{self, render_narrative_pick},
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
    pub config: NarraverseConfig,
    pub state: NarraverseState,
    pub verses: NarraverseVerses,
    pub cached_verse_name: String,
    pub list_states: NarraverseListStates,
    pub editor: NarraverseEditor,
    pub verse_edit_modal: VerseEdit,
    pub narrative_edit_modal: NarrativeEditModal,
    pub entity_edit_modal: EntityEditModal,
    pub artifact_edit_modal: ArtifactEditModal,
    pub config_edit_modal: ConfigEditModal,
    pub deletion_target: DeletionTarget,
    pub errors: (String, NarraverseState),
    pub footer_notification: Option<String>,
    pub cache: NarraverseCaches,
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
    Error,
    Quit,
}

impl Narraverse {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while self.state != NarraverseState::Quit {
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
        match self.state {
            NarraverseState::VersePick => render_worldpick(self, frame, main_layout[1]),
            NarraverseState::VerseEdit => render_verse_edit(self, frame, main_layout[1]),
            NarraverseState::VerseAreaPick => render_area_pick(self, frame, main_layout[1]),
            NarraverseState::NarrativePick => render_narrative_pick(self, frame, main_layout[1]),
            NarraverseState::NarrativeModal => {
                render_narrative_edit_modal(self, frame, main_layout[1])
            }
            NarraverseState::EntityPick => render_entity_pick(self, frame, main_layout[1]),
            NarraverseState::EntityModal => render_entity_edit(self, frame, main_layout[1]),
            NarraverseState::ArtifactPick => render_artifact_pick(self, frame, main_layout[1]),
            NarraverseState::ArtifactModal => render_artifact_edit(self, frame, main_layout[1]),
            NarraverseState::ConfigModal => render_config_edit_modal(self, frame, main_layout[1]),
            NarraverseState::Editor => render_editor(self, frame, main_layout[1]),
            NarraverseState::VerseDelete => render_delete_screen(self, frame, main_layout[1]),
            NarraverseState::Error => render_error_screen(self, frame, main_layout[1]),
            _ => todo!(),
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
        match self.state {
            NarraverseState::VersePick => versepick::handle_keys(self, key),
            NarraverseState::VerseEdit => verseedit::handle_keys(self, key),
            NarraverseState::VerseAreaPick => verseareapick::handle_keys(self, key),
            NarraverseState::NarrativePick => versenarrativepick::handle_keys(self, key),
            NarraverseState::NarrativeModal => narrativeeditmodal::handle_keys(self, key),
            NarraverseState::EntityPick => entitypick::handle_keys(self, key),
            NarraverseState::EntityModal => entityedit::handle_keys(self, key),
            NarraverseState::ArtifactPick => artifactpick::handle_keys(self, key),
            NarraverseState::ArtifactModal => artifacteditmodal::handle_keys(self, key),
            NarraverseState::ConfigModal => configeditmodal::handle_keys(self, key),
            NarraverseState::Editor => nveditor::handle_keys(self, key),
            NarraverseState::VerseDelete => versedelete::handle_keys(self, key),
            NarraverseState::Error => error::handle_keys(self, key),
            _ => {}
        }
    }

    pub fn send_error(&mut self, message: String, return_state: NarraverseState) {
        self.errors.0 = message;
        self.errors.1 = return_state;
        self.state = NarraverseState::Error;
    }

    pub fn send_footer_notification(&mut self, message: String) {
        self.footer_notification = Some(message);
    }
}

impl Default for Narraverse {
    fn default() -> Self {
        Self {
            config: NarraverseConfig::default(),
            state: NarraverseState::VersePick,
            verses: NarraverseVerses::default(),
            cached_verse_name: String::from("enmkdev.neocities.org"),
            list_states: NarraverseListStates::default(),
            editor: NarraverseEditor::default(),
            verse_edit_modal: VerseEdit::default(),
            narrative_edit_modal: NarrativeEditModal::default(),
            entity_edit_modal: EntityEditModal::default(),
            artifact_edit_modal: ArtifactEditModal::default(),
            config_edit_modal: ConfigEditModal::default(),
            deletion_target: DeletionTarget::Verse,
            errors: (String::new(), NarraverseState::VersePick),
            footer_notification: None,
            cache: NarraverseCaches::default(),
        }
    }
}

pub struct NarraverseListStates {
    pub verse_pick_liststate: ListState,
    pub verse_area_pick_liststate: ListState,
    pub verse_narrative_pick_liststate: ListState,
    pub verse_entity_pick_liststate: ListState,
    pub verse_artifact_pick_liststate: ListState,
}

impl Default for NarraverseListStates {
    fn default() -> Self {
        Self {
            verse_pick_liststate: ListState::default(),
            verse_area_pick_liststate: ListState::default().with_selected(Some(0)),
            verse_narrative_pick_liststate: ListState::default(),
            verse_entity_pick_liststate: ListState::default(),
            verse_artifact_pick_liststate: ListState::default(),
        }
    }
}

#[derive(Default)]
pub struct NarraverseCaches {
    pub narrative_list_cache: Vec<String>,
    pub entity_list_cache: Vec<String>,
    pub artifact_list_cache: Vec<String>,
}
