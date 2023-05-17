use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use crate::Language;
use crate::model::{PlWordNet, Metadata};


impl Debug for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::PL => write!(f, "Language::PL"),
            Language::EN => write!(f, "Language::EN"),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::PL => write!(f, "Polish"),
            Language::EN => write!(f, "English"),
        }
    }
}

impl PlWordNet {
    /// Retrieves the metadata of the PlWordNet instance.
    ///
    /// This method returns a `Metadata` struct containing information about the `PlWordNet`
    /// instance, such as the owner, date, version, and counts of elements contained within
    /// such as lexical units, synsets, relation types, lexical relations, and synset relations.
    pub fn get_metadata(&self) -> Metadata {
        Metadata {
            owner: Cow::from(&self.owner),
            date: Cow::from(&self.date),
            version: Cow::from(&self.version),
            lexical_units: self.lexical_units.len(),
            synsets: self.synsets.len(),
            relation_types: self.relation_types.len(),
            lexical_relations: self.lexical_relations.len(),
            synset_relations: self.synset_relations.len(),
        }
    }
}
