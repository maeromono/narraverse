use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
};

use crate::apcore::apversemain::render_header;

pub struct ApVerse {
    pub state: ApVerseState,
}

#[derive(PartialEq, Eq)]
pub enum ApVerseState {
    Editor,
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
        let main_layout1 =
            Layout::horizontal([Constraint::Length(20), Constraint::Fill(1)]).split(main_layout[1]);
        render_header(self, frame, main_layout[0]);
        match self.state {
            _ => todo!()
        }
    }

    fn handle_keys(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.state = ApVerseState::Quit,
            _ => match self.state {
                _ => {}
            }
        }
    }
}

impl Default for ApVerse {
    fn default() -> Self {
        Self {
            state: ApVerseState::Editor,
        }
    }
}
