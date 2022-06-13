use super::{Statement, Token};
use Token::*;

impl Statement {
    pub fn statedef_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let items: Vec<&str> = statement.trim().split_ascii_whitespace().collect();
        let mut tokens = Vec::with_capacity(items.len());
        for (i, item) in items.into_iter().enumerate() {
            match i {
                0 if item == "+" => tokens.push(Operator(item.to_string())),
                _ => {
                    if item == "//" {
                        tokens.push(Operator(item.to_string()));
                        break;
                    } else if item.chars().all(|c| {
                        // TODO implement state tokens
                        char::is_alphanumeric(c) || c == '(' || c == ')' || c == '{' || c == '}'
                    }) {
                        tokens.push(Operand(item.to_string()));
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(tokens)
    }
}

#[cfg(test)]
mod statedef_parse_statement_test {
    use super::super::{Statement, Token::*};
    #[test]
    fn basic() {
        let line = "    + (A) {B} // Two States";
        let tokens = vec![
            Operator("+".to_string()),
            Operand("(A)".to_string()),
            Operand("{B}".to_string()),
            Operator("//".to_string()),
        ];
        let ad_tokens = Statement::parse_statement(line);
        assert!(ad_tokens.is_some());
        assert_eq!(tokens, ad_tokens.unwrap());
    }
}
