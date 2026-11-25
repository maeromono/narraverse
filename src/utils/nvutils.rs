use std::io::Result;

use crate::{
    editor::bindhelp::BindhelpMode,
    nvcore::{
        narraverse::{Narraverse, NarraverseState},
        save::{save_manifest, save_verse},
        verses::NarraverseVerses,
    },
    utils::cacheutils::rebuild_bindhelp_cache,
};

pub fn enter_bindhelp(nv: &mut Narraverse, mode: BindhelpMode) {
    nv.bindhelp_mode = mode;
    rebuild_bindhelp_cache(&mut nv.cache, mode);
    nv.state = NarraverseState::BindHelp;
}

pub fn save_current_verse(verses: &mut NarraverseVerses) -> Result<()> {
    let verse = verses.current_verse_kv();
    save_verse(verse)?;
    save_manifest(verses)?;
    Ok(())
}
