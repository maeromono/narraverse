mod apcore;

use std::process::exit;

use ratatui::run;

use crate::apcore::apverse::ApVerse;

fn main() {
    let mut av = ApVerse::default();
    if let Err(err) = run(|term| av.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
