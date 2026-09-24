use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::Stylize, widgets::{Paragraph, Wrap}};

use crate::{editor::verseedit::VerseEdit, nvcore::{decosutils::center_area, narraverse::{Narraverse, NarraverseState}, save::save_verses}};

pub fn render_delete_screen(nv: &Narraverse, frame: &mut Frame, area: Rect) {
    let Some(verse) = nv.verses.verses.get(nv.verses.verse_index) else {
        frame.render_widget(Paragraph::new("No Verse at this index.\nHit Esc to cancel.").red().bold(), area);
        return;
    };
    let delete_confirmation_message = Paragraph::new(format!("Delete Verse \"{}\"?\nAll narrative you've weitten will be permanently erased.", verse.name)).wrap(Wrap{trim:false}).yellow().bold();
    frame.render_widget(delete_confirmation_message, center_area(area));
}

pub fn handle_keys(nv: &mut Narraverse, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('n') => {
            nv.state = NarraverseState::VersePick;
        }
        KeyCode::Char('y') => {
            confirm_delete(nv);
        }
        _ => {}
    }
}

fn confirm_delete(nv: &mut Narraverse) {
    let mut verse_index = nv.verses.verse_index;
    nv.verses.verses.remove(verse_index);
    if nv.verses.verses.len() == 0 {
        nv.verse_editor = VerseEdit::default();
        nv.state = NarraverseState::VerseEdit;
    } else if verse_index >= nv.verses.verses.len() {
        verse_index -= 1;
        nv.verses.verse_index = verse_index;
        nv.state = NarraverseState::VersePick;
    } else {
        nv.state = NarraverseState::VersePick;
    }
    save_verses(&nv.verses);
}
