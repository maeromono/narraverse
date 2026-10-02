use chrono::NaiveDate;
use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle, previous_cycle};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;
use uuid::Uuid;

use crate::nvcore::{
    cacheutils::rebuild_narr_list_cache, decosutils::{center_area, focus_highlight}, narraverse::{Narraverse, NarraverseState}, save::save_verse, textareautils::{validate_date, validate_length}, verses::{Narrative, NarraverseVerses}
};

pub struct NarrativeEditModal {
    mode: NarrativeEditModalMode,
    current_focus: CurrentFocus,
    narrative_name_textarea: TextArea<'static>,
    narrative_writtenby_textarea: TextArea<'static>,
    narrative_date_textarea: TextArea<'static>,
    narrative_summary_textarea: TextArea<'static>,
    narrative_name_valid: bool,
    narrative_writtenby_valid: bool,
    narrative_date_valid: bool,
}

impl NarrativeEditModal {
    pub fn default() -> Self {
        Self {
            mode: NarrativeEditModalMode::New,
            current_focus: CurrentFocus::Title,
            narrative_name_textarea: TextArea::default(),
            narrative_writtenby_textarea: TextArea::default(),
            narrative_date_textarea: TextArea::default(),
            narrative_summary_textarea: TextArea::default(),
            narrative_name_valid: false,
            narrative_writtenby_valid: false,
            narrative_date_valid: false,
        }
    }

    pub fn edit(verses: &NarraverseVerses, uuid: Uuid) -> Option<Self> {
        let verse = verses.verses.get(&verses.verse_uuid)?;
        let narrative = verse.narratives.get(&uuid)?;
        Some(Self {
            mode: NarrativeEditModalMode::Existing(uuid),
            current_focus: CurrentFocus::Title,
            narrative_name_textarea: TextArea::new(vec![narrative.title.clone()]),
            narrative_writtenby_textarea: TextArea::new(vec![narrative.written_by.clone()]),
            narrative_date_textarea: TextArea::new(vec![narrative.date.to_string()]),
            narrative_summary_textarea: TextArea::new(vec![
                narrative.summary.clone().unwrap_or_default(),
            ]),
            narrative_name_valid: true,
            narrative_writtenby_valid: true,
            narrative_date_valid: true,
        })
    }

    pub fn prefill_writtenby_field(mut self, nv: &Narraverse) -> Self {
        self.narrative_writtenby_textarea =
            TextArea::new(vec![nv.config.default_written_by.clone()]);
        self.narrative_writtenby_valid =
            validate_length(&mut self.narrative_writtenby_textarea, 1, 100);
        self
    }
}

enum NarrativeEditModalMode {
    New,
    Existing(Uuid),
}

#[derive(Sequence)]
pub enum CurrentFocus {
    Title,
    WrittenBy,
    Date,
    Summary,
}

pub fn render_narrative_edit_modal(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let (direction, vert_constraint) = if area.width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    };
    let nem_layout = Layout::vertical([vert_constraint; 4]).split(center_area(area));
    let field_layout = Layout::default()
        .direction(direction)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)]);
    let title_field_layout = field_layout.clone().split(nem_layout[0]);
    let writtenby_field_layout = field_layout.clone().split(nem_layout[1]);
    let date_field_layout = field_layout.clone().split(nem_layout[2]);
    let summary_field_layout = field_layout.clone().split(nem_layout[3]);
    let focus = &nv.narrative_edit_modal.current_focus;
    frame.render_widget(
        Paragraph::new("Narrative title")
            .style(focus_highlight(matches!(focus, CurrentFocus::Title))),
        title_field_layout[0],
    );
    frame.render_widget(
        &nv.narrative_edit_modal.narrative_name_textarea,
        title_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Written by")
            .style(focus_highlight(matches!(focus, CurrentFocus::WrittenBy))),
        writtenby_field_layout[0],
    );
    frame.render_widget(
        &nv.narrative_edit_modal.narrative_writtenby_textarea,
        writtenby_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Narrative date (ISO)")
            .style(focus_highlight(matches!(focus, CurrentFocus::Date))),
        date_field_layout[0],
    );
    frame.render_widget(
        &nv.narrative_edit_modal.narrative_date_textarea,
        date_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Summary (optional)")
            .style(focus_highlight(matches!(focus, CurrentFocus::Summary))),
        summary_field_layout[0],
    );
    frame.render_widget(
        &nv.narrative_edit_modal.narrative_summary_textarea,
        summary_field_layout[1],
    );
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            nv.state = NarraverseState::NarrativePick;
            reset_textareas(nv);
        }
        KeyCode::Tab => {
            nv.narrative_edit_modal.current_focus =
                next_cycle(&nv.narrative_edit_modal.current_focus)
        }
        KeyCode::BackTab => {
            nv.narrative_edit_modal.current_focus =
                previous_cycle(&nv.narrative_edit_modal.current_focus)
        }
        KeyCode::Enter => confirm_edit(nv),
        _ => match &nv.narrative_edit_modal.current_focus {
            CurrentFocus::Title => {
                let textarea = &mut nv.narrative_edit_modal.narrative_name_textarea;
                if textarea.input(*key) {
                    nv.narrative_edit_modal.narrative_name_valid =
                        validate_length(textarea, 1, 100);
                }
            }
            CurrentFocus::WrittenBy => {
                let textarea = &mut nv.narrative_edit_modal.narrative_writtenby_textarea;
                if textarea.input(*key) {
                    nv.narrative_edit_modal.narrative_writtenby_valid =
                        validate_length(textarea, 1, 100);
                }
            }
            CurrentFocus::Date => {
                let textarea = &mut nv.narrative_edit_modal.narrative_date_textarea;
                if textarea.input(*key) {
                    nv.narrative_edit_modal.narrative_date_valid = validate_date(textarea);
                }
            }
            CurrentFocus::Summary => {
                nv.narrative_edit_modal
                    .narrative_summary_textarea
                    .input(*key);
            }
        },
    }
}

fn confirm_edit(nv: &mut Narraverse) {
    if !(nv.narrative_edit_modal.narrative_name_valid
        && nv.narrative_edit_modal.narrative_writtenby_valid
        && nv.narrative_edit_modal.narrative_date_valid)
    {
        return;
    }
    let Some(verse) = nv.verses.verses.get_key_value_mut(&nv.verses.verse_uuid) else {
        nv.send_error(
            format!("No Verse with this UUID ({}) exists.", nv.verses.verse_uuid),
            NarraverseState::VersePick,
        );
        return;
    };
    let sum = nv.narrative_edit_modal.narrative_summary_textarea.lines()[0].clone();
    let summary = if sum.is_empty() { None } else { Some(sum) };
    match nv.narrative_edit_modal.mode {
        NarrativeEditModalMode::New => {
            verse.1.narratives.insert(
                Uuid::now_v7(),
                Narrative {
                    title: nv.narrative_edit_modal.narrative_name_textarea.lines()[0].clone(),
                    written_by: nv.narrative_edit_modal.narrative_writtenby_textarea.lines()[0]
                        .clone(),
                    date: nv.narrative_edit_modal.narrative_date_textarea.lines()[0]
                        .clone()
                        .parse::<NaiveDate>()
                        .unwrap(),
                    summary,
                    narrative: String::new(),
                },
            );
            nv.state = NarraverseState::NarrativePick;
        }
        NarrativeEditModalMode::Existing(uuid) => {
            let Some(narrative) = verse.1.narratives.get_mut(&uuid) else {
                nv.send_error(
                    format!("No narrative with this UUID ({}) exists.", uuid),
                    NarraverseState::VersePick,
                );
                return;
            };
            narrative.title = nv.narrative_edit_modal.narrative_name_textarea.lines()[0].clone();
            narrative.written_by =
                nv.narrative_edit_modal.narrative_writtenby_textarea.lines()[0].clone();
            narrative.date = nv.narrative_edit_modal.narrative_date_textarea.lines()[0]
                .clone()
                .parse::<NaiveDate>()
                .unwrap();
            narrative.summary = summary;
            nv.state = NarraverseState::NarrativePick;
        }
    }
    verse.1.narratives.sort_by(|_, narr1, _, narr2| narr1.date.cmp(&narr2.date));
    if let Err(err) = save_verse(&verse.1, format!("verse-{}", verse.0).as_str()) {
        nv.send_error(
            format!("Narrative edited, but won't save:\n{}", err),
            NarraverseState::NarrativePick,
        );
    } else {
        nv.send_footer_notification(String::from("Narrative edited"));
    }
    reset_textareas(nv);
    rebuild_narr_list_cache(nv);
}

fn reset_textareas(nv: &mut Narraverse) {
    nv.narrative_edit_modal.narrative_name_textarea = TextArea::default();
}
