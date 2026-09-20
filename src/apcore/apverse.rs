use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
};

pub struct ApVerse {
    pub state: ApVerseState,
}

#[derive(PartialEq, Eq)]
pub enum ApVerseState {
    Main,
    Editor,
    Quit,
}

impl ApVerse {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while self.state != ApVerseState::Quit {
            term.draw(|frame| self.render(frame))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {}
}
