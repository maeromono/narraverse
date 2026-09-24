use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Stylize},
    widgets::{List, ListState, Paragraph, Wrap},
};

use crate::nvcore::narraverse::Narraverse;

pub fn render_error_screen(nv: &mut Narraverse, frame: &mut Frame, area: Rect) {
    let error_layout = Layout::vertical([Constraint::Max(3), Constraint::Fill(1)]).split(area);
    let error_paragraph = Paragraph::new(
        "Some errors have occurred at runtime. Review them, then press Esc to go back.",
    )
    .red()
    .bold()
    .wrap(Wrap { trim: true });
    let mut error_list: Vec<String> = vec![];
    for error in &nv.errors.0 {
        error_list.push(error.to_string());
    }
    let list = List::new(error_list).highlight_style(Modifier::REVERSED);
    frame.render_widget(error_paragraph, error_layout[0]);
    frame.render_stateful_widget(list, error_layout[1], &mut nv.errors.1);
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = nv.errors.2.clone();
            nv.errors.0.clear();
            nv.errors.1 = ListState::default().with_selected(Some(0));
        }
        KeyCode::Char('j') | KeyCode::Down => nv.errors.1.select_next(),
        KeyCode::Char('k') | KeyCode::Up => nv.errors.1.select_previous(),
        _ => {}
    }
}
