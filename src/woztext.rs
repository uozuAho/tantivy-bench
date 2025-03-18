use tantivy::schema::{IndexRecordOption, TextFieldIndexing, TextOptions};
use tantivy::tokenizer::RegexTokenizer;

pub const TOKENIZER_NAME: &'static str = "wozregex";

pub fn options() -> TextOptions {
    let indexing = TextFieldIndexing::default()
        .set_tokenizer(TOKENIZER_NAME)
        // todo: look at options. positions allow phrase queries. can do just freqs, but queries fail complaining about positions
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    TextOptions::default()
        .set_indexing_options(indexing)
}

pub fn tokenizer() -> tantivy::Result<RegexTokenizer> {
    RegexTokenizer::new(r"(?:\w)")
}
