#[cfg(test)]
mod tests {
    use tantivy::tokenizer::{RegexTokenizer, TextAnalyzer, Token};

    #[test]
    fn test_simple() {
        let tokens = token_stream_helper("just some words", r"(?:\w+)");
        assert_eq!(tokens.len(), 3);
        assert_eq!(&tokens[0].text, "just");
        assert_eq!(&tokens[1].text, "some");
        assert_eq!(&tokens[2].text, "words");
    }

    #[test]
    fn test_punctuation() {
        let tokens = token_stream_helper("a,then_b.c'd", r"(?:\w+)");
        assert_eq!(tokens.len(), 4);
        assert_eq!(&tokens[0].text, "a");
        assert_eq!(&tokens[1].text, "then_b");
        assert_eq!(&tokens[2].text, "c");
        assert_eq!(&tokens[3].text, "d");
    }

    #[test]
    fn test_slash() {
        let tokens = token_stream_helper("a/b/c", r"(?:\w+)");
        assert_eq!(tokens.len(), 3);
        assert_eq!(&tokens[0].text, "a");
        assert_eq!(&tokens[1].text, "b");
        assert_eq!(&tokens[2].text, "c");
    }

    fn token_stream_helper(text: &str, pattern: &str) -> Vec<Token> {
        let r = RegexTokenizer::new(pattern).unwrap();
        let mut a = TextAnalyzer::from(r);
        let mut token_stream = a.token_stream(text);
        let mut tokens: Vec<Token> = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        token_stream.process(&mut add_token);
        tokens
    }
}
