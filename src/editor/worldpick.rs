use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::apcore::apverse::ApVerse;

pub fn render_worldpick(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new("I'm bricked up"), area);
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {

}
