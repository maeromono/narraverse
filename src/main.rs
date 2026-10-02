mod editor;
mod nvcore;

use std::process::exit;

use ratatui::run;
use uuid::Uuid;

use crate::nvcore::{
    cacheutils::{rebuild_art_list_cache, rebuild_ent_list_cache, rebuild_narr_list_cache},
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
    if nv.verses.verse_uuid != Uuid::default() {
        nv.state = NarraverseState::VerseAreaPick;
        rebuild_narr_list_cache(&mut nv);
        rebuild_ent_list_cache(&mut nv);
        rebuild_art_list_cache(&mut nv);
    }
    if let Err(err) = run(|term| nv.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
