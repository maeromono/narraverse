use crate::nvcore::{
    editorbindhelp::{BindhelpMode, EDITOR_BINDINGS, PICKMENU_BINDINGS},
    narraverse::Narraverse,
};

pub fn rebuild_narr_list_cache(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        return;
    };
    nv.cache.narrative_list_cache.clear();
    for narr in verse.narratives.values() {
        nv.cache
            .narrative_list_cache
            .push(format!("[{}] {}", narr.date, narr.title));
    }
}

pub fn rebuild_ent_list_cache(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        return;
    };
    nv.cache.entity_list_cache.clear();
    for ent in verse.entities.values() {
        nv.cache
            .entity_list_cache
            .push(format!("[{}] {}", ent.entity_type.short_name(), ent.name));
    }
}

pub fn rebuild_art_list_cache(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        return;
    };
    nv.cache.artifact_list_cache.clear();
    for art in verse.artifacts.values() {
        nv.cache
            .artifact_list_cache
            .push(format!("[{}] {}", art.artifact_type.name(), art.name));
    }
}

pub fn rebuild_bindhelp_cache(nv: &mut Narraverse) {
    nv.cache.bindhelp_bindings_cache.clear();
    let binds = match nv.bindhelp_mode {
        BindhelpMode::Pickmenu => PICKMENU_BINDINGS,
        BindhelpMode::Editor => EDITOR_BINDINGS,
    };
    for (bind, desc) in binds {
        nv.cache
            .bindhelp_bindings_cache
            .push(format!("{}: {}", bind, desc));
    }
}
