use chrono::NaiveDate;
use ratatui::style::Style;
use ratatui_textarea::TextArea;

pub fn validate_length(textarea: &mut TextArea, min: usize, max: usize) -> bool {
    let length = textarea.lines()[0].len();
    if length > max || length < min {
        textarea.set_style(Style::default().red().reversed());
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

pub fn validate_date(textarea: &mut TextArea) -> bool {
    let text = &textarea.lines()[0];
    if let Err(_) = text.parse::<NaiveDate>() {
        textarea.set_style(Style::default().red().reversed());
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

pub fn searchbox_style(textarea: &mut TextArea, res: Result<(), regex::Error>) {
    if let Err(_) = res {
        textarea.set_style(Style::default().red().reversed());
    } else {
        textarea.set_style(Style::default());
    }
}
