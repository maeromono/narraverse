use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
};

use crate::nvcore::narraverse::{Narraverse, NarraverseState};

pub fn render_header(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let header_layout =
        Layout::horizontal([Constraint::Length(20), Constraint::Fill(1)]).split(area);
    let name_ver = Paragraph::new(format!("narraverse v{}", env!("CARGO_PKG_VERSION")));
    let right_thing = Paragraph::new(match nv.get_state() {
        NarraverseState::VersePick | NarraverseState::ConfigModal => "maero.surge.sh",
        _ => &nv.get_verse_manifest().current_verse().name,
    })
    .right_aligned();
    frame.render_widget(name_ver, header_layout[0]);
    frame.render_widget(right_thing, header_layout[1]);
}

pub fn render_footer(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let bind_help_string = match nv.get_state() {
        NarraverseState::VersePick => "e edit, n new, j/k navigate, Enter select",
        NarraverseState::NarrativeModal
        | NarraverseState::EntityModal
        | NarraverseState::ArtifactModal => "Esc cancel, Tab cycle, Enter confirm",
        NarraverseState::VerseEdit | NarraverseState::ConfigModal => "Esc cancel, Enter confirm",
        NarraverseState::VerseAreaPick => "j/k navigate, Enter select",
        NarraverseState::NarrativePick
        | NarraverseState::EntityPick
        | NarraverseState::ArtifactPick => "e edit, n new, j/k navigate, Enter select",
        NarraverseState::Editor => "Esc back, Alt+x close, Alt+w save, F1 binds",
        NarraverseState::VerseDelete => "Esc cancel, y confirm",
        NarraverseState::BindHelp => "Esc back",
        NarraverseState::Error => "Esc back, j/k scroll",
        _ => "",
    };
    let bind_help = Paragraph::new(bind_help_string).centered();
    frame.render_widget(bind_help, area);
}
