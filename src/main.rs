mod editor;
mod nvcore;
mod utils;

use std::process::exit;

use ratatui::run;
use uuid::Uuid;

use crate::{
    nvcore::{
        config::{NarraverseConfig, load_config},
        narraverse::{Narraverse, NarraverseState},
        save::load_verses,
        verses::NarraverseVerses,
    },
    utils::cacheutils::{
        rebuild_art_list_cache, rebuild_ent_list_cache, rebuild_narr_list_cache,
        rebuild_verse_list_cache,
    },
};

fn main() {
    let mut nv = Narraverse::default();
    let config = load_config().unwrap_or_else(|err| {
        nv.send_error(
            format!("Unable to parse config file:\n{}", err),
            NarraverseState::VersePick,
        );
        NarraverseConfig::default()
    });
    nv.replace_config(config);
    let errors: Option<Vec<String>>;
    (nv.verses, errors) = load_verses().unwrap_or_else(|err| {
        nv.send_error(
            format!("Verse file contains invalid data:\n{}", err),
            NarraverseState::VersePick,
        );
        (NarraverseVerses::default(), None)
    });
    if let Some(load_errors) = errors {
        nv.send_error(load_errors.join("\n"), NarraverseState::VersePick);
    }
    rebuild_verse_list_cache(&mut nv.cache, &nv.verses);
    if nv.verses.verse_uuid != Uuid::default()
        && nv.verses.verses.contains_key(&nv.verses.verse_uuid)
    {
        nv.set_state(NarraverseState::VerseAreaPick);
        rebuild_narr_list_cache(&mut nv.cache, &nv.verses);
        rebuild_ent_list_cache(&mut nv.cache, &nv.verses);
        rebuild_art_list_cache(&mut nv.cache, &nv.verses);
    }
    if let Err(err) = run(|term| nv.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
