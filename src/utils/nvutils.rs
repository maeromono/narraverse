use std::io::Result;

use uuid::Uuid;

use crate::{editor::bindhelp::BindhelpMode, nvcore::{narraverse::{Narraverse, NarraverseState}, save::save_verse, verses::Verse}, utils::cacheutils::rebuild_bindhelp_cache};

pub fn enter_bindhelp(nv: &mut Narraverse, mode: BindhelpMode) {
    nv.bindhelp_mode = mode;
    rebuild_bindhelp_cache(&mut nv.cache, mode);
    nv.state = NarraverseState::BindHelp;
}

pub fn save_whole_verse(verse: (&Uuid, &mut Verse)) -> Result<()> {
    verse.1.changed = true;
    save_verse(verse.1, format!("verse-{}", verse.0).as_str())?;
    Ok(())
}
