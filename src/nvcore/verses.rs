use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NarraverseVerses {
    pub verses: Vec<Verse>,
    pub verse_index: usize,
}

impl Default for NarraverseVerses {
    fn default() -> Self {
        Self {
            verses: vec![],
            verse_index: 0,
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Verse {
    pub name: String,
    pub narratives: Vec<Narrative>,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Narrative {
    pub title: String,
    pub narrative: String,
}
