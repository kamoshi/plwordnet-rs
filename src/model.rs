use std::collections::{HashMap};


/// Represents the language of lexical units and synsets in plWordNet.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Language {
    /// Indicates Polish language.
    PL,
    /// Indicates English language.
    EN,
}

/// Represents the plWordNet lexical resource.
#[derive(Debug)]
pub struct PlWordNet {
    pub(crate) owner: String,
    pub(crate) date: String,
    pub(crate) version: String,
    pub(crate) lexical_units: HashMap<usize, LexicalUnit>,
    pub(crate) synsets: HashMap<usize, Synset>,
    pub(crate) relation_types: HashMap<usize, RelationType>,
    pub(crate) lexical_relations: Vec<LexicalRelation>,
    pub(crate) synset_relations: Vec<SynsetRelation>,
}

/// Metadata information for a PlWordNet instance.
#[derive(Debug, Clone)]
pub struct Metadata<'a> {
    pub owner: &'a str,
    pub date: &'a str,
    pub version: &'a str,
    pub lexical_units: usize,
    pub synsets: usize,
    pub relation_types: usize,
    pub lexical_relations: usize,
    pub synset_relations: usize,
}

#[derive(Debug)]
pub(crate) struct LexicalUnit {
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

/// Represents a readonly view of a lexical unit.
#[derive(Debug, Clone)]
pub struct LexicalUnitView<'a> {
    pub id: usize,
    pub name: &'a str,
    pub pos: &'a str,
    pub tagcount: i32,
    pub domain: &'a str,
    pub desc: &'a str,
    pub workstate: &'a str,
    pub source: &'a str,
    pub variant: i32,
    pub language: Language,
}

#[derive(Debug)]
pub(crate) struct Synset {
    pub id: usize,
    pub workstate: String,
    pub split: i32,
    pub owner: String,
    pub definition: String,
    pub desc: String,
    pub abstract_: bool,
    pub lexical_units: Vec<usize>,
}

/// Represents a readonly view of a synset.
#[derive(Debug)]
pub struct SynsetView<'a> {
    pub id: usize,
    pub workstate: &'a str,
    pub split: i32,
    pub owner: &'a str,
    pub definition: &'a str,
    pub desc: &'a str,
    pub abstract_: bool,
    pub lexical_units: Vec<LexicalUnitView<'a>>,
    pub language: Language,
}

#[derive(Debug)]
pub(crate) struct RelationType {
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
pub(crate) struct RelationTypeTest {
    pub text: String,
    pub pos: String,
}

#[derive(Debug)]
pub struct RelationTypeView<'a> {
    pub id: usize,
    pub type_: &'a str,
    pub reverse: usize,
    pub name: &'a str,
    pub description: &'a str,
    pub posstr: &'a str,
    pub display: &'a str,
    pub shortcut: &'a str,
    pub autoreverse: bool,
    pub pwn: &'a str,
    // TODO: RelationTypeTest
}

#[derive(Debug)]
pub(crate) struct LexicalRelation {
    pub parent: usize,
    pub child: usize,
    /// Type of the relation
    pub relation: usize,
    pub valid: bool,
    pub owner: String,
}

/// Represents a readonly view of a lexical relation.
#[derive(Debug)]
pub struct LexicalRelationView<'a> {
    pub parent: Option<LexicalUnitView<'a>>,
    pub child: Option<LexicalUnitView<'a>>,
    /// Type of the relation
    pub relation: Option<RelationTypeView<'a>>,
    pub valid: bool,
    pub owner: &'a str,
}

#[derive(Debug)]
pub(crate) struct SynsetRelation {
    pub parent: usize,
    pub child: usize,
    pub relation: usize,
    pub valid: bool,
    pub owner: String,
}

/// Represents a readonly view of a synset relation.
#[derive(Debug)]
pub struct SynsetRelationView<'a> {
    pub parent: Option<SynsetView<'a>>,
    pub child: Option<SynsetView<'a>>,
    /// Type of the relation
    pub relation: Option<RelationTypeView<'a>>,
    pub valid: bool,
    pub owner: &'a str,
}
