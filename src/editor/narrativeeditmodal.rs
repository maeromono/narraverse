use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{ListState, Paragraph},
};
use ratatui_textarea::TextArea;

use crate::nvcore::{
    narraverse::{Narraverse, NarraverseState},
    decosutils::center_area,
    save::save_verses,
    verses::Narrative,
};

pub struct NarrativeEditModal {
    mode: NarrativeEditModalMode,
    narrative_name_textarea: TextArea<'static>,
    narrative_name_valid: bool,
}

impl NarrativeEditModal {
    pub fn default() -> Self {
        Self {
            mode: NarrativeEditModalMode::New,
            narrative_name_textarea: TextArea::default(),
            narrative_name_valid: false,
        }
    }

    pub fn edit(nv: &Narraverse, ind: usize) -> Option<Self> {
        let verse = nv.verses.verses.get(ind)?;
        Some(Self {
            mode: NarrativeEditModalMode::Existing(ind),
            narrative_name_textarea: TextArea::new(vec![verse.name.clone()]),
            narrative_name_valid: true,
        })
    }
}

enum NarrativeEditModalMode {
    New,
    Existing(usize),
}

pub fn render_narrative_edit_modal(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = if area.width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    };
    let ve_layout = Layout::vertical([vert_constraint; 1]) // bcuz more coming soon
        .split(center_area(area));
    let field_layout = Layout::default()
        .direction(direction)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)]);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    frame.render_widget(Paragraph::new("Narrative title"), name_field_layout[0]);
    frame.render_widget(
        &nv.narrative_edit_modal.narrative_name_textarea,
        name_field_layout[1],
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::VersePick;
            reset_textareas(nv);
        }
        KeyCode::Enter => confirm_edit(nv),
        _ => {
            let textarea = &mut nv.narrative_edit_modal.narrative_name_textarea;
            if textarea.input(*key) {
                nv.narrative_edit_modal.narrative_name_valid = validate_name(textarea);
            }
        }
    }
}

fn validate_name(textarea: &mut TextArea) -> bool {
    let length = textarea.lines()[0].len();
    if length > 100 || length < 1 {
        textarea.set_style(Style::default().fg(Color::Black).bg(Color::Red));
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !nv.narrative_edit_modal.narrative_name_valid {
        return;
    }
    let Some(verse) = nv.verses.verses.get_mut(nv.verses.verse_index) else {
        nv.state = NarraverseState::VersePick;
        return;
    };
    match nv.narrative_edit_modal.mode {
        NarrativeEditModalMode::New => {
            verse.narratives.push(Narrative {
                title: nv.narrative_edit_modal.narrative_name_textarea.lines()[0].clone(),
                narrative: String::new()
            });
            nv.list_states.verse_narrative_pick_liststate = ListState::default().with_selected(Some(0));
            nv.list_states.verse_narrative_pick_liststate.select_last();
            nv.state = NarraverseState::NarrativePick;
        }
        NarrativeEditModalMode::Existing(index) => {
            let Some(narrative) = verse.narratives.get_mut(index) else {
                return;
            };
            narrative.title = nv.narrative_edit_modal.narrative_name_textarea.lines()[0].clone();
            nv.state = NarraverseState::NarrativePick;
        }
    }
    save_verses(&nv.verses);
    reset_textareas(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.narrative_edit_modal.narrative_name_textarea = TextArea::default();
}
