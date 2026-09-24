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
        narrativeedit::{self, NarrativeEditor, render_narrative_editor},
        narrativeeditmodal::{self, NarrativeEditModal, render_narrative_edit_modal},
        verseareapick::{self, render_area_pick},
        versedelete::{self, DeletionTarget, render_delete_screen},
        verseedit::{self, VerseEdit, render_verse_edit},
        versenarrativepick::{self, render_narrative_pick},
        versepick::{self, render_worldpick},
    },
    nvcore::{
        error::{self, render_error_screen}, narraversemain::{render_footer, render_header}, verses::NarraverseVerses
    },
};

pub struct Narraverse {
    pub state: NarraverseState,
    pub verses: NarraverseVerses,
    pub cached_verse_name: String,
    pub list_states: NarraverseListStates,
    pub verse_editor: VerseEdit,
    pub narrative_edit_modal: NarrativeEditModal,
    pub narrative_editor: NarrativeEditor,
    pub deletion_target: DeletionTarget,
    pub errors: (Option<String>, NarraverseState),
}

#[derive(Clone, PartialEq, Eq)]
pub enum NarraverseState {
    VersePick,
    VerseEdit,
    VerseAreaPick,
    NarrativePick,
    NewNarrative,
    NarrativeEdit,
    EntityPick,
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
            NarraverseState::NewNarrative => {
                render_narrative_edit_modal(self, frame, main_layout[1])
            }
            NarraverseState::NarrativeEdit => render_narrative_editor(self, frame, main_layout[1]),
            NarraverseState::VerseDelete => render_delete_screen(self, frame, main_layout[1]),
            NarraverseState::Error => render_error_screen(self, frame, main_layout[1]),
            _ => todo!(),
        }
        render_header(self, frame, main_layout[0]);
        render_footer(self, frame, main_layout[2]);
    }

    fn handle_keys(&mut self, key: &KeyEvent) {
        match self.state {
            NarraverseState::VersePick => versepick::handle_keys(self, key),
            NarraverseState::VerseEdit => verseedit::handle_keys(self, key),
            NarraverseState::VerseAreaPick => verseareapick::handle_keys(self, key),
            NarraverseState::NarrativePick => versenarrativepick::handle_keys(self, key),
            NarraverseState::NewNarrative => narrativeeditmodal::handle_keys(self, key),
            NarraverseState::NarrativeEdit => narrativeedit::handle_keys(self, key),
            NarraverseState::VerseDelete => versedelete::handle_keys(self, key),
            NarraverseState::Error => error::handle_keys(self, key),
            _ => {}
        }
    }

    pub fn send_error(&mut self, message: String, state_to_jump: NarraverseState) {
        self.errors.0 = Some(message);
        self.errors.1 = state_to_jump;
        self.state = NarraverseState::Error;
    }
}

impl Default for Narraverse {
    fn default() -> Self {
        Self {
            state: NarraverseState::VersePick,
            verses: NarraverseVerses::default(),
            cached_verse_name: String::from("enmkdev.neocities.org"),
            list_states: NarraverseListStates::default(),
            verse_editor: VerseEdit::default(),
            narrative_edit_modal: NarrativeEditModal::default(),
            narrative_editor: NarrativeEditor::default(),
            deletion_target: DeletionTarget::Verse,
            errors: (None, NarraverseState::VersePick)
        }
    }
}

pub struct NarraverseListStates {
    pub verse_pick_liststate: ListState,
    pub verse_area_pick_liststate: ListState,
    pub verse_narrative_pick_liststate: ListState,
}

impl Default for NarraverseListStates {
    fn default() -> Self {
        let ls = ListState::default().with_selected(Some(0));
        Self {
            verse_pick_liststate: ls.clone(),
            verse_area_pick_liststate: ls.clone(),
            verse_narrative_pick_liststate: ls.clone(),
        }
    }
}
