use once_cell::sync::Lazy;
use crate::model::PlWordNet;


pub(crate) static WORDNET: Lazy<PlWordNet> = Lazy::new(||
    PlWordNet::from_file("plwordnet_4_2.xml").unwrap()
);


#[test]
fn loading() {
    let meta = WORDNET.get_metadata();
    assert_eq!(meta.lexical_units, 513410);
    assert_eq!(meta.synsets, 353585);
    assert_eq!(meta.relation_types, 306);
    assert_eq!(meta.synset_relations, 1477851);
    assert_eq!(meta.lexical_relations, 393137);
}
