pub struct ApVerseVerses {
    pub verses: Vec<Verse>,
    pub verse_index: usize,
}

impl Default for ApVerseVerses {
    fn default() -> Self {
        Self { verses: vec![], verse_index: 0 }
    }
}

pub struct Verse {
    pub name: String,
    pub narrative: String,
}
