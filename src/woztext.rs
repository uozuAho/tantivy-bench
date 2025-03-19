use tantivy::schema::{IndexRecordOption, TextFieldIndexing, TextOptions};
use tantivy::tokenizer::{Language, LowerCaser, RegexTokenizer, RemoveLongFilter, Stemmer, TextAnalyzer};

pub const TOKENIZER_NAME: &'static str = "wozregex";

pub fn options() -> TextOptions {
    let indexing = TextFieldIndexing::default()
        .set_tokenizer(TOKENIZER_NAME)
        // Could maybe save memory here by not saving positions.
        // Positions allow phrase search. I couldn't get queries
        // to work without saving positions.
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    TextOptions::default()
        .set_indexing_options(indexing)
}

pub fn tokenizer() -> TextAnalyzer {
    let regex_tokenizer = RegexTokenizer::new(r"(?:\w+)").unwrap();

    TextAnalyzer::builder(regex_tokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(Stemmer::new(Language::English))
        .build()
}
