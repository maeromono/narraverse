use crate::nvcore::narraverse::Narraverse;

pub fn rebuild_narr_list_cache(nv: &mut Narraverse) {
    let Some(verse) = nv.verses.verses.get(&nv.verses.verse_uuid) else {
        return;
    };
    nv.cache.narrative_list_cache.clear();
    for narr in verse.narratives.values() {
        nv.cache.narrative_list_cache.push(format!("[{}] {}", narr.date, narr.title));
    }
}
