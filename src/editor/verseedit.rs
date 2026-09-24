use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::Paragraph};
use ratatui_textarea::TextArea;

use crate::apcore::{apverse::{ApVerse, ApVerseState}, decosutils::center_area, save::save_verses, verses::Verse};

pub struct VerseEdit {
    mode: VerseEditMode,
    verse_name_textarea: TextArea<'static>,
    verse_name_valid: bool,
}

impl VerseEdit {
    pub fn default() -> Self {
        Self {
            mode: VerseEditMode::New,
            verse_name_textarea: TextArea::default(),
            verse_name_valid: false,
        }
    }

    pub fn edit(av: &ApVerse, ind: usize) -> Option<Self> {
        let verse = av.verses.verses.get(ind)?;
        Some(Self {
            mode: VerseEditMode::Existing(ind),
            verse_name_textarea: TextArea::new(vec![verse.name.clone()]),
            verse_name_valid: true,
        })
    }
}

enum VerseEditMode {
    New,
    Existing(usize),
}

pub fn render_verse_edit(av: &ApVerse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = if area.width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    };
    let ve_layout = Layout::vertical([vert_constraint;1]) // bcuz more coming soon
        .split(center_area(area));
    let field_layout = Layout::default().direction(direction).constraints([Constraint::Fill(1), Constraint::Fill(2)]);
    let name_field_layout = field_layout.clone().split(ve_layout[0]);
    frame.render_widget(Paragraph::new("Verse name"), name_field_layout[0]);
    frame.render_widget(&av.verse_editor.verse_name_textarea, name_field_layout[1]);
}

pub fn handle_keys(av: &mut ApVerse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            av.state = ApVerseState::VersePick;
            reset_textareas(av);
        }
        KeyCode::Enter => confirm_edit(av),
        _ => {
            let textarea = &mut av.verse_editor.verse_name_textarea;
            if textarea.input(*key) {
                av.verse_editor.verse_name_valid = validate_name(textarea);
            }
        }
    }
}

fn validate_name(textarea: &mut TextArea) -> bool {
    let length = textarea.lines()[0].len();
    if length > 50 || length < 1 {
        textarea.set_style(Style::default().fg(Color::Black).bg(Color::Red));
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

fn confirm_edit(av: &mut ApVerse) {
    if !av.verse_editor.verse_name_valid {
        return;
    }
    match av.verse_editor.mode {
        VerseEditMode::New => {
            av.verses.verses.push(Verse {
                name: av.verse_editor.verse_name_textarea.lines()[0].clone(),
                narratives: vec![]
            });
            av.state = ApVerseState::VersePick;
        }
        VerseEditMode::Existing(index) => {
            let Some(verse) = av.verses.verses.get_mut(index) else {
                av.state = ApVerseState::VersePick;
                return;
            };
            verse.name = av.verse_editor.verse_name_textarea.lines()[0].clone();
            av.state = ApVerseState::VersePick;
        }
    }
    save_verses(&av.verses);
    reset_textareas(av);
}

fn reset_textareas(av: &mut ApVerse) {
    av.verse_editor.verse_name_textarea = TextArea::default();
}
