use super::{Statement, Token};
use Token::*;

impl Statement {
    pub fn fadefend_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let items: Vec<&str> = statement.trim().split_ascii_whitespace().collect();
        let mut tokens = Vec::with_capacity(items.len());
        for (i, item) in items.into_iter().enumerate() {
            match i {
                0 if item == "}" => tokens.push(Operator(item.to_string())),
                1 if item == "//" => {
                    tokens.push(Operator(item.to_string()));
                    break;
                }
                _ => return None,
            }
        }
        Some(tokens)
    }
}

#[cfg(test)]
mod fadefend_parse_statement_test {
    use super::super::{Statement, Token::*};
    #[test]
    fn basic() {
        let line = "} // End";
        let tokens = vec![Operator("}".to_string()), Operator("//".to_string())];
        let ad_tokens = Statement::parse_statement(line);
        assert!(ad_tokens.is_some());
        assert_eq!(tokens, ad_tokens.unwrap());
    }
}
