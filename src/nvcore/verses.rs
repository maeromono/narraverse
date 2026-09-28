use chrono::NaiveDate;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NarraverseVerses {
    #[serde(skip)]
    pub verses: Vec<Verse>,
    pub verse_files: Vec<String>,
    pub verse_index: usize,
}

impl NarraverseVerses {
    pub fn refresh_verse_files(&mut self) {
        let mut files_list: Vec<String> = vec![];
        for verse in &self.verses {
            files_list.push(format!("verse-{}", verse.uuid));
        }
        self.verse_files = files_list;
    }
}

impl Default for NarraverseVerses {
    fn default() -> Self {
        Self {
            verses: vec![],
            verse_files: vec![],
            verse_index: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Verse {
    pub name: String,
    pub uuid: Uuid,
    pub narratives: Vec<Narrative>,
    pub entities: Vec<Entity>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Narrative {
    pub title: String,
    pub written_by: String,
    pub narrative: String,
    pub date: NaiveDate,
    pub summary: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
}

#[derive(Clone, Serialize, Deserialize, Sequence)]
pub enum EntityType {
    Character,
    Creature,
    Location,
    Group,
    Organization,
    Corporate,
    Concept,
}

impl EntityType {
    pub fn short_name(&self) -> &'static str {
        match self {
            EntityType::Character => "Char",
            EntityType::Creature => "Cret",
            EntityType::Location => "Loct",
            EntityType::Group => "Grop",
            EntityType::Organization => "Orgn",
            EntityType::Corporate => "Corp",
            EntityType::Concept => "Cnct",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            EntityType::Character => "Character",
            EntityType::Creature => "Creature",
            EntityType::Location => "Location",
            EntityType::Group => "Group",
            EntityType::Organization => "Organization",
            EntityType::Corporate => "Corporate",
            EntityType::Concept => "Concept",
        }
    }
}
