use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{ListState, Paragraph},
};

use crate::{
    apcore::{
        apversemain::render_header,
        verses::{ApVerseVerses, Verse},
    },
    editor::{
        narrativeedit::{self, NarrativeEditor, render_narrative_editor},
        verseareapick::{self, render_area_pick},
        versenarrativepick::{self, render_narrative_pick},
        versepick::{self, render_worldpick},
    },
};

pub struct ApVerse {
    pub state: ApVerseState,
    pub verses: ApVerseVerses,
    pub cached_verse_name: String,
    pub list_states: ApVerseListStates,
    pub narrative_editor: NarrativeEditor,
}

#[derive(PartialEq, Eq)]
pub enum ApVerseState {
    VersePick,
    VerseAreaPick,
    NarrativePick,
    NewNarrative,
    NarrativeEdit,
    EntityPick,
    NewEntity,
    EntityEdit,
    VerseDelete,
    Quit,
}

impl ApVerse {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while self.state != ApVerseState::Quit {
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
            ApVerseState::VersePick => render_worldpick(self, frame, main_layout[1]),
            ApVerseState::VerseAreaPick => render_area_pick(self, frame, main_layout[1]),
            ApVerseState::NarrativePick => render_narrative_pick(self, frame, main_layout[1]),
            ApVerseState::NarrativeEdit => render_narrative_editor(self, frame, main_layout[1]),
            _ => todo!(),
        }
        render_header(self, frame, main_layout[0]);
    }

    fn handle_keys(&mut self, key: &KeyEvent) {
        match self.state {
            ApVerseState::VersePick => versepick::handle_keys(self, key),
            ApVerseState::VerseAreaPick => verseareapick::handle_keys(self, key),
            ApVerseState::NarrativePick => versenarrativepick::handle_keys(self, key),
            ApVerseState::NarrativeEdit => narrativeedit::handle_keys(self, key),
            _ => {}
        }
    }
}

impl Default for ApVerse {
    fn default() -> Self {
        Self {
            state: ApVerseState::VersePick,
            verses: ApVerseVerses::default(),
            cached_verse_name: String::from("enmkdev.neocities.org"),
            list_states: ApVerseListStates::default(),
            narrative_editor: NarrativeEditor::default(),
        }
    }
}

pub struct ApVerseListStates {
    pub verse_pick_liststate: ListState,
    pub verse_area_pick_liststate: ListState,
    pub verse_narrative_pick_liststate: ListState,
}

impl Default for ApVerseListStates {
    fn default() -> Self {
        let ls = ListState::default().with_selected(Some(0));
        Self {
            verse_pick_liststate: ls.clone(),
            verse_area_pick_liststate: ls.clone(),
            verse_narrative_pick_liststate: ls.clone(),
        }
    }
}
