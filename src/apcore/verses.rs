use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApVerseVerses {
    pub verses: Vec<Verse>,
    pub verse_index: usize,
}

impl Default for ApVerseVerses {
    fn default() -> Self {
        Self {
            verses: vec![Verse {
                name: String::from("Loremverse: Origins"),
                narratives: vec![
                    Narrative {
                        title: String::from("Chapter I: The birth of Lorem"),
                        narrative: String::from(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque malesuada ante vel risus hendrerit ultrices.",
                        ),
                    },
                    Narrative {
                        title: String::from("Chapter II: The birth of Ipsum"),
                        narrative: String::from(
                            "Ipsum lorem dolor sit amet, consectetur adipiscing elit. Quisque malesuada ante vel risus hendrerit ultrices.",
                        ),
                    },
                ],
            }],
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
