mod editor;
mod nvcore;

use std::process::exit;

use ratatui::run;

use crate::{
    editor::verseedit::VerseEdit,
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        save::load_verses,
    },
};

fn main() {
    let mut nv = Narraverse::default();
    nv.verses = load_verses().unwrap_or_default();
    if nv.verses.verses.len() == 0 {
        nv.verse_editor = VerseEdit::default();
        nv.state = NarraverseState::VerseEdit;
    }
    if let Err(err) = run(|term| nv.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
