use ratatui::{Frame, layout::{Constraint, Layout, Rect}, widgets::Paragraph};

use crate::apcore::apverse::ApVerse;

pub fn render_header(av: &mut ApVerse, frame: &mut Frame, area: Rect) {
    let header_layout = Layout::horizontal([Constraint::Length(15), Constraint::Fill(1)]).split(area);
    let name_ver = Paragraph::new("apverse v0.1.0");
    let wm_lol = Paragraph::new("enmkdev.neocities.org").right_aligned();
    frame.render_widget(name_ver, header_layout[0]);
    frame.render_widget(wm_lol, header_layout[1]);
}
