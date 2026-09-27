use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
};

use crate::nvcore::narraverse::{Narraverse, NarraverseState};

pub fn render_header(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let header_layout =
        Layout::horizontal([Constraint::Length(20), Constraint::Fill(1)]).split(area);
    let name_ver = Paragraph::new("narraverse v0.2.1");
    let right_thing = Paragraph::new(match nv.state {
        NarraverseState::VersePick => "enmkdev.neocities.org",
        _ => &nv.cached_verse_name,
    })
    .right_aligned();
    frame.render_widget(name_ver, header_layout[0]);
    frame.render_widget(right_thing, header_layout[1]);
}

pub fn render_footer(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let bind_help_string = match nv.state {
        NarraverseState::VersePick => "e edit, n new, j/k navigate, Enter select",
        NarraverseState::VerseEdit
        | NarraverseState::NarrativeModal
        | NarraverseState::EntityModal => "Esc cancel, Tab cycle, Enter confirm",
        NarraverseState::VerseAreaPick => "j/k navigate, Enter select",
        NarraverseState::NarrativePick | NarraverseState::EntityPick => {
            "e edit, n new, j/k navigate, Enter select"
        }
        NarraverseState::NarrativeEdit => "Esc back, Alt+w save",
        NarraverseState::VerseDelete => "n cancel, y confirm",
        _ => "",
    };
    let bind_help = Paragraph::new(bind_help_string).centered();
    frame.render_widget(bind_help, area);
}
