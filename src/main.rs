mod editor;
mod nvcore;

use std::process::exit;

use ratatui::run;

use crate::nvcore::{
        config::load_config, narraverse::Narraverse, save::load_verses
    };

fn main() {
    let mut nv = Narraverse::default();
    nv.verses = load_verses().unwrap_or_default();
    nv.config = load_config().unwrap_or_default();
    if let Err(err) = run(|term| nv.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
