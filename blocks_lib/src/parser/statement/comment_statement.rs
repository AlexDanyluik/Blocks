use super::{Statement, Token};
use Token::*;

impl Statement {
    pub fn comment_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let items: Vec<&str> = statement.trim().split_ascii_whitespace().collect();
        let mut tokens = Vec::with_capacity(items.len());
        if items[0] == "//" {
            tokens.push(Operator(items[0].to_string()));
        }
        Some(tokens)
    }
}

#[cfg(test)]
mod comment_parse_statement_test {
    use super::super::{Statement, Token::*};
    #[test]
    fn basic() {
        let line = "// comment anything";
        let tokens = vec![Operator("//".to_string())];
        let ad_tokens = Statement::parse_statement(line);
        assert!(ad_tokens.is_some());
        assert_eq!(tokens, ad_tokens.unwrap());
    }
}
