use chrono::NaiveDate;
use ratatui::style::{Color, Style};
use ratatui_textarea::TextArea;

pub fn validate_length(textarea: &mut TextArea, min: usize, max: usize) -> bool {
    let length = textarea.lines()[0].len();
    if length > max || length < min {
        textarea.set_style(Style::default().fg(Color::Black).bg(Color::Red));
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

pub fn validate_date(textarea: &mut TextArea) -> bool {
    let text = &textarea.lines()[0];
    if let Err(_) = text.parse::<NaiveDate>() {
        textarea.set_style(Style::default().fg(Color::Black).bg(Color::Red));
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}
