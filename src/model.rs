use std::collections::{HashMap, HashSet};
use std::ops::Not;


/// Represents the plWordNet lexical resource.
#[derive(Debug)]
pub struct PlWordNet {
    pub owner: String,
    pub date: String,
    pub version: String,
    pub lexical_units: HashMap<usize, LexicalUnit>,
    pub synsets: HashMap<usize, Synset>,
    pub relation_types: HashMap<usize, RelationType>,
    pub lexical_relations: Vec<LexicalRelation>,
    pub synset_relations: Vec<SynsetRelation>,
}

#[derive(Debug)]
pub struct LexicalUnit {
    pub id: usize,
    pub name: String,
    pub pos: String,
    pub tagcount: i32,
    pub domain: String,
    pub desc: String,
    pub workstate: String,
    pub source: String,
    pub variant: i32,
    pub language: String,
}

#[derive(Debug)]
pub struct Synset {
    pub id: usize,
    pub workstate: String,
    pub split: i32,
    pub owner: String,
    pub definition: String,
    pub desc: String,
    pub abstract_: bool,
    pub lexical_units: Vec<usize>
}

#[derive(Debug)]
pub struct RelationType {
    pub id: usize,
    pub type_: String,
    pub reverse: usize,
    pub name: String,
    pub description: String,
    pub posstr: String,
    pub display: String,
    pub shortcut: String,
    pub autoreverse: bool,
    pub pwn: String,
    pub tests: Vec<RelationTypeTest>,
}

#[derive(Debug)]
pub struct RelationTypeTest {
    pub text: String,
    pub pos: String,
}

#[derive(Debug)]
pub struct LexicalRelation {
    pub parent: usize,
    pub child: usize,
    /// Type of the relation
    pub relation: usize,
    pub valid: bool,
    pub owner: String,
}

#[derive(Debug)]
pub struct SynsetRelation {
    pub parent: usize,
    pub child: usize,
    /// Type of the relation
    pub relation: usize,
    pub valid: bool,
    pub owner: String,
}


/// Represents the language of lexical units and synsets in plWordNet.
pub enum Language {
    /// Indicates Polish language.
    PL,
    /// Indicates English language.
    EN,
}


impl PlWordNet {
    pub fn filter_synsets_by_lang(&self, lang: Language) -> impl Iterator<Item=&Synset> {
        self.synsets.values()
            .filter_map(move |s| {
                let is_polish = |lu_id: &usize| -> bool {
                    self.lexical_units.get(lu_id).unwrap().language.eq("pl")
                };
                match lang {
                    Language::PL => s.lexical_units.iter().all(is_polish).then(|| s),
                    Language::EN => s.lexical_units.iter().all(is_polish).not().then(|| s),
                }
            }
            )
    }

    pub fn synset_relations_by_id(&self, id: usize) -> impl Iterator<Item=&SynsetRelation> {
        self.synset_relations.iter()
            .filter_map(move |rel| match rel.relation == id {
                true => Some(rel),
                false => None,
            })
    }

    pub fn lexical_units_for_synset(&self, id: usize) -> impl Iterator<Item=&LexicalUnit> {
        self.synsets.get(&id).unwrap()
            .lexical_units
            .iter()
            .filter_map(|lu_id| self.lexical_units.get(lu_id))
    }

    pub fn lexical_units_for_synsets<'data, 'a>(&'data self, ids: &'a HashSet<usize>) -> impl Iterator<Item=&LexicalUnit> + 'a
        where 'data : 'a
    {
        ids.iter()
            .filter_map(|id| self.synsets.get(id))
            .flat_map(|s| s.lexical_units.iter())
            .filter_map(move |lu| self.lexical_units.get(lu))
    }

    pub fn synset_to_simple(&self, id: usize) -> String {
        let synset = self.synsets.get(&id).unwrap();
        let iter = synset.lexical_units.iter()
            .filter_map(|lu_id| self.lexical_units.get(lu_id));

        let mut buffer = String::new();
        for lu in iter {
            if buffer.len() > 0 { buffer.push(',') };
            buffer.push_str(lu.to_simple());
        }
        buffer
    }

    pub fn synsets_to_simple(&self, ids: &HashSet<usize>) -> String {
        let mut buffer = String::new();
        for &id in ids {
            if buffer.len() > 0 { buffer.push(',') };
            buffer.push_str(&self.synset_to_simple(id))
        }
        buffer
    }
}


impl LexicalUnit {
    fn to_simple(&self) -> &str {
        &self.name
    }
}
