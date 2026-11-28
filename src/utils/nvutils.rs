use std::io::Result;

use crate::{
    editor::bindhelp::BindhelpMode,
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        save::{save_manifest, save_verse},
        versemanifest::VerseManifest,
    },
};

pub fn enter_bindhelp(nv: &mut Narraverse, mode: BindhelpMode) {
    nv.bindhelp_mode = mode;
    nv.set_state(NarraverseState::BindHelp);
}

pub fn save_current_verse(verses: &mut VerseManifest) -> Result<()> {
    let verse = verses.current_verse_kv();
    save_verse(verse)?;
    save_manifest(verses)?;
    Ok(())
}
