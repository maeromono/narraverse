mod editor;
mod nvcore;

use std::process::exit;

use ratatui::run;

use crate::nvcore::{
    config::{NarraverseConfig, load_config},
    narraverse::{Narraverse, NarraverseState},
    save::load_verses,
    verses::NarraverseVerses,
};

fn main() {
    let mut nv = Narraverse::default();
    nv.config = load_config().unwrap_or_else(|err| {
        nv.send_error(
            format!("Unable to parse config file:\n{}", err),
            NarraverseState::VersePick,
        );
        NarraverseConfig::default()
    });
    nv.verses = load_verses().unwrap_or_else(|err| {
        nv.send_error(
            format!("Verse file contains invalid data:\n{}", err),
            NarraverseState::VersePick,
        );
        NarraverseVerses::default()
    });
    if let Err(err) = run(|term| nv.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
