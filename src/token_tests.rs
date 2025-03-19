#[cfg(test)]
mod tests {
    use crate::woztext;

    use tantivy::tokenizer::{Token, TokenizerManager};

    fn tokens(text: &str) -> Vec<String> {
        let tokenizer_manager = TokenizerManager::default();
        tokenizer_manager.register("asdf", woztext::tokenizer());
        let mut tokenizer = tokenizer_manager.get("asdf").unwrap();
        let mut tokens: Vec<Token> = vec![];
        {
            let mut add_token = |token: &Token| {
                tokens.push(token.clone());
            };
            tokenizer
                .token_stream(text)
                .process(&mut add_token);
        }
        tokens.iter().map(|t| t.text.clone()).collect()
    }

    #[test]
    fn stems() {
        assert_eq!(
            tokens("library libraries bikes fishing argue arguing"),
            ["librari", "librari", "bike", "fish", "argu", "argu"]);
    }
}
