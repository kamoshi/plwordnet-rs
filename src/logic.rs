use std::fmt::{Debug, Display, Formatter};
use crate::Language;
use crate::model::{
    PlWordNet,
    Metadata,
    LexicalUnitView,
    LexicalUnit,
    SynsetView,
    Synset,
    SynsetRelationView,
    LexicalRelationView,
    RelationType,
    RelationTypeView
};


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
            owner: &self.owner,
            date: &self.date,
            version: &self.version,
            lexical_units: self.lexical_units.len(),
            synsets: self.synsets.len(),
            relation_types: self.relation_types.len(),
            lexical_relations: self.lexical_relations.len(),
            synset_relations: self.synset_relations.len(),
        }
    }

    /// Retrieves a lexical unit with the specified ID from the plWordNet.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the lexical unit to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `LexicalUnitView` if a lexical unit with the given ID is found,
    /// or `None` if no lexical unit exists with the specified ID.
    pub fn get_lexical_unit(&self, id: usize) -> Option<LexicalUnitView> {
        self.lexical_units.get(&id).map(move |lu| lu.into())
    }

    /// Returns an iterator over the lexical units in the plWordNet.
    ///
    /// The iterator yields `LexicalUnitView` instances, providing access to each lexical unit's
    /// information.
    ///
    /// # Returns
    ///
    /// An iterator that yields `LexicalUnitView` instances representing the lexical units in the
    /// plWordNet.
    pub fn iter_lexical_units(&self) -> impl Iterator<Item=LexicalUnitView> {
        self.lexical_units.values().map(move |lu| lu.into())
    }

    /// Retrieves a synset with the specified ID from the plWordNet.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the synset to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `SynsetView` if a synset with the given ID is found,
    /// or `None` if no synset exists with the specified ID.
    pub fn get_synset(&self, id: usize) -> Option<SynsetView> {
        self.synsets.get(&id).map(|s| synset_to_view(&self, s))
    }

    /// Returns an iterator over the synsets in the plWordNet.
    ///
    /// The iterator yields `SynsetView` instances, providing access to each synset's information.
    ///
    /// # Returns
    ///
    /// An iterator that yields `SynsetView` instances representing the synsets in the plWordNet.
    pub fn iter_synsets(&self) -> impl Iterator<Item=SynsetView> {
        self.synsets.values().map(move |s| synset_to_view(&self, s))
    }

    /// Returns an iterator over the lexical relations in the plWordNet.
    ///
    /// The iterator yields `LexicalRelationView` instances, providing access to each lexical relation's
    /// information.
    ///
    /// # Returns
    ///
    /// An iterator that yields `LexicalRelationView` instances representing the lexical relations in
    /// the plWordNet.
    pub fn iter_lexical_relations(&self) -> impl Iterator<Item=LexicalRelationView> {
        self.lexical_relations.iter().map(|lr| LexicalRelationView {
            parent: self.get_lexical_unit(lr.parent),
            child: self.get_lexical_unit(lr.child),
            relation: self.get_relation_type(lr.relation),
            valid: lr.valid,
            owner: &lr.owner,
        })
    }

    /// Returns an iterator over the synset relations in the plWordNet.
    ///
    /// The iterator yields `SynsetRelationView` instances, providing access to each synset relation's
    /// information.
    ///
    /// # Returns
    ///
    /// An iterator that yields `SynsetRelationView` instances representing the synset relations in
    /// the plWordNet.
    pub fn iter_synset_relations(&self) -> impl Iterator<Item=SynsetRelationView> {
        self.synset_relations.iter().map(|lr| SynsetRelationView {
            parent: self.get_synset(lr.parent),
            child: self.get_synset(lr.child),
            relation: self.get_relation_type(lr.relation),
            valid: lr.valid,
            owner: &lr.owner,
        })
    }

    /// Retrieves a relation type with the specified ID from the plWordNet.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the relation type to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `RelationTypeView` if a relation type with the given ID is found,
    /// or `None` if no relation type exists with the specified ID.
    pub fn get_relation_type(&self, id: usize) -> Option<RelationTypeView> {
        self.relation_types.get(&id).map(|rt| rt.into())
    }

    /// Returns an iterator over the relation types in the plWordNet.
    ///
    /// The iterator yields `RelationTypeView` instances, providing access to each relation type's
    /// information.
    ///
    /// # Returns
    ///
    /// An iterator that yields `RelationTypeView` instances representing the relation types in the
    /// plWordNet.
    pub fn iter_relation_types(&self) -> impl Iterator<Item=RelationTypeView> {
        self.relation_types.values().map(|rt| rt.into())
    }
}

impl<'a> From<&'a LexicalUnit> for LexicalUnitView<'a> {
    fn from(lu: &'a LexicalUnit) -> Self {
        Self {
            id: lu.id,
            name: &lu.name,
            pos: &lu.pos,
            tagcount: lu.tagcount,
            domain: &lu.domain,
            desc: &lu.desc,
            workstate: &lu.workstate,
            source: &lu.source,
            variant: lu.variant,
            language: if lu.pos.ends_with(" pwn") { Language::EN } else { Language::PL },
        }
    }
}

impl<'a> From<&'a RelationType> for RelationTypeView<'a> {
    fn from(rt: &'a RelationType) -> Self {
        Self {
            id: rt.id,
            type_: &rt.type_,
            reverse: rt.reverse,
            name: &rt.name,
            description: &rt.description,
            posstr: &rt.posstr,
            display: &rt.display,
            shortcut: &rt.shortcut,
            autoreverse: rt.autoreverse,
            pwn: &rt.pwn,
        }
    }
}

fn synset_to_view<'a>(wn: &'a PlWordNet, s: &'a Synset) -> SynsetView<'a> {
    let lus: Vec<_> = s.lexical_units.iter()
        .filter_map(|&id| wn.get_lexical_unit(id))
        .collect();
    let language = lus.first().map_or(Language::PL, |lu| lu.language);
    SynsetView {
        id: s.id,
        workstate: &s.workstate,
        split: s.split,
        owner: &s.owner,
        definition: &s.definition,
        desc: &s.desc,
        abstract_: s.abstract_,
        lexical_units: lus,
        language
    }
}
