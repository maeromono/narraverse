use crate::{
    editor::bindhelp::{BindhelpMode, EDITOR_BINDINGS, PICKMENU_BINDINGS},
    nvcore::{narraverse::NarraverseCaches, verses::NarraverseVerses},
};

pub fn rebuild_verse_list_cache(cache: &mut NarraverseCaches, verses: &NarraverseVerses) {
    cache.verse_list_cache.clear();
    for verse in verses.verses.values() {
        cache
            .verse_list_cache
            .push(verse.name.to_string());
    }
}

pub fn rebuild_narr_list_cache(cache: &mut NarraverseCaches, verses: &NarraverseVerses) {
    let verse = verses.current_verse();
    cache.narrative_list_cache.clear();
    for narr in verse.narratives.values() {
        cache
            .narrative_list_cache
            .push(format!("[{}] {}", narr.date, narr.title));
    }
}

pub fn rebuild_ent_list_cache(cache: &mut NarraverseCaches, verses: &NarraverseVerses) {
    let verse = verses.current_verse();
    cache.entity_list_cache.clear();
    for ent in verse.entities.values() {
        cache
            .entity_list_cache
            .push(format!("[{}] {}", ent.entity_type.short_name(), ent.name));
    }
}

pub fn rebuild_art_list_cache(cache: &mut NarraverseCaches, verses: &NarraverseVerses) {
    let verse = verses.current_verse();
    cache.artifact_list_cache.clear();
    for art in verse.artifacts.values() {
        cache
            .artifact_list_cache
            .push(format!("[{}] {}", art.artifact_type.name(), art.name));
    }
}

pub fn rebuild_bindhelp_cache(cache: &mut NarraverseCaches, mode: BindhelpMode) {
    cache.bindhelp_bindings_cache.clear();
    let binds = match mode {
        BindhelpMode::Pickmenu => PICKMENU_BINDINGS,
        BindhelpMode::Editor => EDITOR_BINDINGS,
    };
    for (bind, desc) in binds {
        cache
            .bindhelp_bindings_cache
            .push(format!("{}: {}", bind, desc));
    }
}
