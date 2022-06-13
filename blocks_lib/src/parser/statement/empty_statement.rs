use super::{Statement, Token};
use Token::*;

impl Statement {
    pub fn empty_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let statement = statement.trim();
        if statement.is_empty() {
            return Some(vec![Operator("".to_string())]);
        }
        None
    }
}

#[cfg(test)]
mod empty_parse_statement_test {
    use super::super::{Statement, Token::*};
    #[test]
    fn basic() {
        let line = "    ";
        let tokens = vec![Operator("".to_string())];
        let ad_tokens = Statement::parse_statement(line);
        assert!(ad_tokens.is_some());
        assert_eq!(tokens, ad_tokens.unwrap());
    }
}
