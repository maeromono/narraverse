use chrono::NaiveDate;
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Verse {
    pub name: String,
    pub narratives: Vec<Narrative>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Narrative {
    pub title: String,
    pub written_by: String,
    pub narrative: String,
    pub date: NaiveDate,
    pub summary: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum EntityType {
    Character,
    Creature,
    Location,
    Item,
    Group,
    Organization,
    Corporate,
    Concept
}
