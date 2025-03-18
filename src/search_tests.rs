#[cfg(test)]
mod tests {
    use crate::woztext;

    use tantivy::{doc, Index, IndexWriter, ReloadPolicy};
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::{Schema};

    // this is probably heavy, meh I dunno what I'm doing
    fn it_finds(query: &str, doc_text: &str) -> bool {
        let mut schema_builder = Schema::builder();
        // todo: why does my indexer not find anything?
        let text_field = schema_builder.add_text_field("text", woztext::options());

        let index = Index::create_in_ram(schema_builder.build());
        index.tokenizers().register(woztext::TOKENIZER_NAME, woztext::tokenizer());

        let mut index_writer: IndexWriter = index.writer(15_000_000).unwrap();
        index_writer.add_document(doc!(text_field => doc_text)).expect("TODO: panic message");
        index_writer.commit().expect("this must work");

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .unwrap();
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![text_field]);
        let query = query_parser.parse_query(query).unwrap();
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

        top_docs.len() == 1

        //index_writer.delete_all_documents() // todo: maybe use this for perf
    }

    fn it_does_not_find(query: &str, doc_text: &str) -> bool { !it_finds(query, doc_text) }

    #[test]
    fn test_single_word() {
        assert!(it_finds("ham", "the ham is good"));
    }

    #[test]
    fn test_does_not_find_missing_word() {
        assert!(it_does_not_find("pizza", "the ham is good"));
    }

    #[test]
    fn test_finds_simple_plural() {
        assert!(it_finds("bike", "I own several bikes"));
    }

    #[test]
    fn test_finds_harder_plural() {
        assert!(it_finds("library", "There are many libraries"));
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
}
