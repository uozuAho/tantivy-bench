use tantivy::{doc, Index, IndexWriter, Term};
use tantivy::schema::{Schema, TEXT};

// just a copy of another test to get the hang of rust/this lib
#[test]
fn test_some_docs() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();
    let text_field = schema_builder.add_text_field("text", TEXT);
    let index = Index::create_in_ram(schema_builder.build());

    let mut index_writer: IndexWriter = index.writer(300_000_000)?;
    for _ in 0..100 {
        index_writer.add_document(doc!(text_field => "a"))?;
    }
    index_writer.commit()?;
    for _ in 0..100 {
        index_writer.add_document(doc!(text_field => "b"))?;
    }
    assert!(index_writer.commit().is_ok());

    let num_docs_containing = |s: &str| {
        let term_a = Term::from_field_text(text_field, s);
        index.reader()?.searcher().doc_freq(&term_a)
    };
    assert_eq!(num_docs_containing("a")?, 100);
    assert_eq!(num_docs_containing("b")?, 100);
    Ok(())
}

// this is probably heavy, meh I dunno what I'm doing
fn it_finds(query: &str, doc_text: &str) -> bool {
    let mut schema_builder = Schema::builder();
    let text_field = schema_builder.add_text_field("text", TEXT);
    let index = Index::create_in_ram(schema_builder.build());

    let mut index_writer: IndexWriter = index.writer(300_000_000).unwrap();
    index_writer.add_document(doc!(text_field => doc_text)).expect("TODO: panic message");
    index_writer.commit().expect("this must work");
    //index_writer.delete_all_documents() // todo: maybe use this for perf

    let term_a = Term::from_field_text(text_field, query);
    index.reader().unwrap().searcher().doc_freq(&term_a).unwrap() == 1
}

#[test]
fn test_single_word() {
    assert!(it_finds("ham", "the ham is good"));
}

#[test]
fn test_does_not_find_missing_word() {
    assert!(!it_finds("pizza", "the ham is good"));
}

#[test]
fn test_finds_stemmed_word() {
    assert!(!it_finds("bike", "I own several bikes"));
}

#[test]
fn test_finds_word_before_slash() {
    assert!(!it_finds("red", "red/green/refactor"));
}

#[test]
fn test_finds_word_between_slashes() {
    assert!(!it_finds("green", "red/green/refactor"));
}

#[test]
fn test_finds_word_after_slash() {
    assert!(!it_finds("refactor", "red/green/refactor"));
}