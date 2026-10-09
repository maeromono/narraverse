use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

use crate::{nvcore::narraverse::Narraverse, utils::decosutils::center_area};

pub fn render_error_screen(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let error_layout =
        Layout::vertical([Constraint::Max(2), Constraint::Fill(1)]).split(center_area(area));
    let error_paragraph =
        Paragraph::new("An error has occurred. Review it, then press Esc to go back.")
            .red()
            .bold()
            .wrap(Wrap { trim: false });
    let error_message_paragraph = Paragraph::new(nv.errors.0.clone()).wrap(Wrap { trim: false });
    frame.render_widget(error_paragraph, error_layout[0]);
    frame.render_widget(error_message_paragraph, error_layout[1]);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = nv.errors.1.clone();
            nv.errors.0 = String::new();
        }
        _ => {}
    }
}
