pub struct ApVerseVerses {
    pub verses: Vec<Verse>,
    pub verse_index: usize,
}

impl Default for ApVerseVerses {
    fn default() -> Self {
        Self {
            verses: vec![
                Verse {
                    name: String::from("Loremverse"),
                    narrative: String::from(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque malesuada ante vel risus hendrerit ultrices.",
                    ),
                },
                Verse {
                    name: String::from("Loremverse II"),
                    narrative: String::from(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque malesuada ante vel risus hendrerit ultrices. but better. or worse. depending if it's from a passionate writer or from a soulless corpo or an incel",
                    ),
                },
            ],
            verse_index: 0,
        }
    }
}

pub struct Verse {
    pub name: String,
    pub narrative: String,
}
