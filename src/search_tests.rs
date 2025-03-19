#[cfg(test)]
mod tests {
    use crate::woztext;

    use tantivy::{doc, Index, IndexWriter, ReloadPolicy, TantivyDocument};
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::{Schema, Value, STORED, STRING};

    struct DummyDoc {
        path: String,
        contents: String
    }

    impl DummyDoc {
        fn new(path: &str, contents: &str) -> DummyDoc {
            DummyDoc { path: path.to_string(), contents: contents.to_string() }
        }
    }

    // this is probably heavy, meh I dunno what I'm doing
    fn search_docs(query: &str, docs: Vec<DummyDoc>) -> Vec<String> {
        let mut schema_builder = Schema::builder();
        let path_field = schema_builder.add_text_field("path", STRING | STORED);
        let content_field = schema_builder.add_text_field("content", woztext::options());

        let index = Index::create_in_ram(schema_builder.build());
        index.tokenizers().register(woztext::TOKENIZER_NAME, woztext::tokenizer());

        let mut index_writer: IndexWriter = index.writer(15_000_000).unwrap();
        for doc in docs {
            index_writer.add_document(doc!(
                path_field => doc.path,
                content_field => doc.contents,
            )).expect("failed to add doc");
        }
        index_writer.commit().expect("failed to commit index");

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .unwrap();
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![content_field]);
        let query = query_parser.parse_query(query).unwrap();
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

        let mut paths = vec![];

        for (_, addr) in top_docs {
            let doc: TantivyDocument = searcher.doc(addr).unwrap();
            paths.push(doc.get_first(path_field).unwrap().as_str().unwrap().to_owned());
        }

        paths
    }

    fn it_finds(query: &str, doc_text: &str) -> bool {
        let docs = search_docs(query, vec![DummyDoc::new("", doc_text)]);
        docs.len() == 1
    }

    fn it_does_not_find(query: &str, doc_text: &str) -> bool { !it_finds(query, doc_text) }

    #[test]
    fn test_find_doc() {
        let found_docs = search_docs("blah", vec![
            DummyDoc::new("a/b.txt", "blah blah some stuff and things"),
            DummyDoc::new("a/c/d.txt", "what about shoes and biscuits"),
        ]);
        assert_eq!(found_docs, ["a/b.txt"]);
    }

    #[test]
    fn test_single_word() {
        assert!(it_finds("ham", "the ham is good"));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(it_finds("HAM", "the ham is good"));
    }

    #[test]
    fn test_case_insensitive2() {
        assert!(it_finds("ham", "the HAM is good"));
    }

    #[test]
    fn test_does_not_find_missing_word() {
        assert!(it_does_not_find("pizza", "the ham is good"));
    }

    #[test]
    fn test_stem_bikes() {
        assert!(it_finds("bike", "I own several bikes"));
    }

    #[test]
    fn test_stem_libraries() {
        assert!(it_finds("library", "There are many libraries"));
    }

    #[test]
    fn test_stem_argue() {
        assert!(it_finds("argue", "Stop arguing"));
    }

    #[test]
    fn test_finds_word_before_slash() {
        assert!(it_finds("red", "red/green/refactor"));
    }

    #[test]
    fn test_finds_word_between_slashes() {
        assert!(it_finds("green", "red/green/refactor"));
    }

    #[test]
    fn test_finds_word_after_slash() {
        assert!(it_finds("refactor", "red/green/refactor"));
    }

    #[test]
    fn test_md_single() {
        assert!(it_finds("bike", "I have a [bike](a/b/c)"));
    }

    #[test]
    fn test_md_first_word() {
        assert!(it_finds("ham", "I have a [ham bike](a/b/c)"));
    }

    #[test]
    fn test_md_middle_word() {
        assert!(it_finds("ham", "I have a [super ham bike](a/b/c)"));
    }

    #[test]
    fn test_md_last_word() {
        assert!(it_finds("bike", "I have a [super ham bike](a/b/c)"));
    }

    #[test]
    fn test_query_search_terms_are_or() {
        assert!(it_finds("ham potato", "plenty of ham"));
    }

    #[test]
    fn test_term_presence() {
        assert!(it_finds("+ham", "plenty of ham"));
    }

    #[test]
    fn test_term_presence_rejects_missing_words() {
        assert!(it_does_not_find("ham +beach", "plenty of ham"));
    }

    #[test]
    fn test_term_presence_ignores_missing_negative() {
        assert!(it_finds("ham -beach", "plenty of ham"));
    }

    #[test]
    fn test_term_presence_excludes_matched_negative() {
        assert!(it_does_not_find("ham -plenty", "plenty of ham"));
    }
}
