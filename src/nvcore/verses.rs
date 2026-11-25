use chrono::NaiveDate;
use enum_iterator::Sequence;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NarraverseVerses {
    #[serde(skip)]
    pub verses: IndexMap<Uuid, Verse>,
    pub verse_files: IndexMap<Uuid, String>,
    pub verse_uuid: Uuid,
}

impl NarraverseVerses {
    pub fn current_verse(&self) -> &Verse {
        self.verses
            .get(&self.verse_uuid)
            .expect("Invalid Verse UUID")
    }

    pub fn current_verse_mut(&mut self) -> &mut Verse {
        self.verses
            .get_mut(&self.verse_uuid)
            .expect("Invalid Verse UUID")
    }

    pub fn current_verse_kv(&self) -> (&Uuid, &Verse) {
        self.verses
            .get_key_value(&self.verse_uuid)
            .expect("Invalid Verse UUID")
    }

    pub fn current_verse_kv_mut(&mut self) -> (&Uuid, &mut Verse) {
        self.verses
            .get_key_value_mut(&self.verse_uuid)
            .expect("Invalid Verse UUID")
    }
}

impl Default for NarraverseVerses {
    fn default() -> Self {
        Self {
            verses: IndexMap::default(),
            verse_files: IndexMap::default(),
            verse_uuid: Uuid::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Verse {
    pub name: String,
    pub narratives: IndexMap<Uuid, Narrative>,
    pub entities: IndexMap<Uuid, Entity>,
    pub artifacts: IndexMap<Uuid, Artifact>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Narrative {
    pub title: String,
    pub written_by: String,
    pub date: NaiveDate,
    pub narrative: String,
    pub summary: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub details: String,
}

#[derive(Clone, Serialize, Deserialize, Sequence)]
pub enum EntityType {
    Character,
    Creature,
    Location,
    Group,
    Organization,
    Corporate,
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
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub artifact_type: ArtifactType,
    pub details: String,
}

#[derive(Clone, Serialize, Deserialize, Sequence)]
pub enum ArtifactType {
    Item,
    Concept,
    Note,
}

impl ArtifactType {
    pub fn name(&self) -> &'static str {
        match self {
            ArtifactType::Item => "Item",
            ArtifactType::Concept => "Concept",
            ArtifactType::Note => "Note",
        }
    }
}
