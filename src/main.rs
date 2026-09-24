mod apcore;
mod editor;

use std::process::exit;

use ratatui::run;

use crate::apcore::{apverse::ApVerse, save::load_verses};

fn main() {
    let mut av = ApVerse::default();
    av.verses = load_verses().unwrap_or_default();
    if let Err(err) = run(|term| av.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
