use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::model::*;


fn cast_string(text: Cow<[u8]>) -> String {
    String::from_utf8_lossy(text.as_ref()).to_string()
}

fn cast_usize(text: Cow<[u8]>) -> usize {
    cast_string(text).parse().unwrap()
}

fn cast_i32(text: Cow<[u8]>) -> i32 {
    cast_string(text).parse().unwrap()
}

fn cast_bool(text: Cow<[u8]>) -> bool {
    cast_string(text).eq("true")
}

macro_rules! gen_branch_test {
    ($x:ident) => { bstringify::bstringify!($x) };
    ($x:ident, $test:literal) => { $test };
}

macro_rules! gen_cast {
    ($expr:expr) => { cast_string($expr) };
    ($expr:expr, usize) => { cast_usize($expr) };
    ($expr:expr, i32) => { cast_i32($expr) };
    ($expr:expr, bool) => { cast_bool($expr) };
    ($expr:expr, _) => { cast_string($expr) };
}

macro_rules! gen_parser {
    (
        $name:ident,
        $structure:ident,
        $( {$($mapped:ident),*} ,)?
        $( [$($nested:ident),*] ,)?
        $($x:ident $(: $test:literal)? $(-> $ty:tt)? ),*
    ) => {
        fn $name(event: &BytesStart) -> $structure {
            $(let mut $x = None;)*
            for attr in event.attributes() {
                let attr = attr.unwrap();
                match attr.key.0 {
                    $(gen_branch_test!($x $(, $test)?) => $x = Some(gen_cast!(attr.value $(, $ty)?)),)*
                    _ => ()
                };
            }
            $structure {
                $( $x: $x.unwrap_or(Default::default()), )*
                $( $( $nested: Vec::new(), )* )?
                $( $( $mapped: HashMap::new(), )* )?
            }
        }
    }
}


gen_parser!(parse_array_list,
    PlWordnet,
    {lexical_units, synsets, relation_types},
    [lexical_relations, synset_relations],
    owner,
    date,
    version
);

gen_parser!(parse_lexical_unit,
    LexicalUnit,
    id -> usize,
    name,
    pos,
    tagcount -> i32,
    domain,
    desc,
    workstate,
    source,
    variant -> i32,
    language
);

gen_parser!(parse_synset,
    Synset,
    [lexical_units],
    id -> usize,
    workstate,
    split -> i32,
    owner,
    definition,
    desc,
    abstract_: b"abstract" -> bool
);

gen_parser!(parse_relation_type,
    RelationType,
    [tests],
    id -> usize,
    type_: b"type",
    reverse -> usize,
    name,
    description,
    posstr,
    display,
    shortcut,
    autoreverse -> bool,
    pwn
);

gen_parser!(parse_relation_type_test,
    RelationTypeTest,
    text,
    pos
);

gen_parser!(parse_lexical_relation,
    LexicalRelation,
    parent -> usize,
    child -> usize,
    relation -> usize,
    valid -> bool,
    owner
);

gen_parser!(parse_synset_relation,
    SynsetRelation,
    parent -> usize,
    child -> usize,
    relation -> usize,
    valid -> bool,
    owner
);


const TAG_ARRAY_LIST: &[u8] = b"array-list";
const TAG_LEXICAL_UNIT: &[u8] = b"lexical-unit";
const TAG_SYNSET: &[u8] = b"synset";
const TAG_RELATION_TYPE: &[u8] = b"relationtypes";
const TAG_RELATION_TYPE_TEST: &[u8] = b"test";
const TAG_LEXICAL_RELATION: &[u8] = b"lexicalrelations";
const TAG_SYNSET_RELATION: &[u8] = b"synsetrelations";
const TAG_UNIT_ID: &[u8] = b"unit-id";


enum ParsingContext {
    None,
    Synset(usize),
    RelationType(usize),
}


fn from_file(path: &str) -> Result<PlWordnet, Box<dyn Error>> {
    let mut reader = Reader::from_file(path)?;
    let mut buf = Vec::new();

    let mut root: Option<PlWordnet> = None;
    let mut context = ParsingContext::None;
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            // <node attribute="" />
            Ok(Event::Empty(e)) => {
                match e.name().as_ref() {
                    TAG_LEXICAL_UNIT => {
                        let mut data = parse_lexical_unit(&e);
                        data.language = match data.pos.ends_with(" pwn") {
                            true => "en".to_string(),
                            false => "pl".to_string(),
                        };
                        root.as_mut().unwrap().lexical_units.insert(data.id, data);
                    },
                    TAG_RELATION_TYPE_TEST => {
                        let data = parse_relation_type_test(&e);
                        match context {
                            ParsingContext::RelationType(id) => {
                                root.as_mut().unwrap().relation_types
                                    .entry(id)
                                    .and_modify(|e| e.tests.push(data));
                            },
                            _ => unreachable!()
                        }
                    },
                    TAG_LEXICAL_RELATION => {
                        let data = parse_lexical_relation(&e);
                        root.as_mut().unwrap().lexical_relations.push(data);
                    },
                    TAG_SYNSET_RELATION => {
                        let data = parse_synset_relation(&e);
                        root.as_mut().unwrap().synset_relations.push(data)
                    },
                    TAG_RELATION_TYPE => {
                        let data = parse_relation_type(&e);
                        root.as_mut().unwrap().relation_types.insert(data.id.to_owned(), data);
                    },
                    _ => (),
                }
            },
            // <node attribute="">
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    TAG_ARRAY_LIST => root = Some(parse_array_list(&e)),
                    TAG_SYNSET => {
                        let data = parse_synset(&e);
                        context = ParsingContext::Synset(data.id);
                        root.as_mut().unwrap().synsets.insert(data.id.to_owned(), data);
                    },
                    TAG_RELATION_TYPE => {
                        let data = parse_relation_type(&e);
                        context = ParsingContext::RelationType(data.id);
                        root.as_mut().unwrap().relation_types.insert(data.id.to_owned(), data);
                    },
                    TAG_UNIT_ID => (),
                    _ => unreachable!(),
                }
            }
            // Text
            Ok(Event::Text(event)) => match context {
                ParsingContext::Synset(id) => {
                    let text = event.unescape().unwrap().into_owned();
                    let lu_id = match text.trim() {
                        "" => continue,
                        num => num.parse().unwrap()
                    };
                    root.as_mut().unwrap().synsets
                        .entry(id)
                        .and_modify(|e| e.lexical_units.push(lu_id));
                },
                _ => (),
            }
            _ => (),
        }
    }
    Ok(root.unwrap())
}


impl PlWordnet {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        from_file(path)
    }
}
