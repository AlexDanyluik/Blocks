use super::{Statement, Token};

impl Statement {
    pub fn fadef_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let items: Vec<&str> = statement.trim().split_ascii_whitespace().collect();
        let mut tokens = Vec::with_capacity(items.len());
        for (i, item) in items.into_iter().enumerate() {
            match i {
                0 if item == "FA" => tokens.push(Token::Operator(item.to_owned())),
                1 if item.chars().all(char::is_alphabetic) => {
                    tokens.push(Token::Operand(item.to_owned()))
                }
                2 if item == "{" => tokens.push(Token::Operator(item.to_owned())),
                3 if item == "//" => {
                    tokens.push(Token::Operator(item.to_owned()));
                    break;
                }
                _ => return None,
            }
        }
        Some(tokens)
    }
}

#[cfg(test)]
mod fadef_parse_statement_tests {
    use super::{Statement, Token};

    #[test]
    fn basic() {
        let line = "  FA LastOne { // ending with one";
        let tokens = vec![
            Token::Operator("FA".to_string()),
            Token::Operand("LastOne".to_string()),
            Token::Operator("{".to_string()),
            Token::Operator("//".to_string()),
        ];
        let fa_tokens = Statement::parse_statement(line);
        assert!(fa_tokens.is_some());
        assert_eq!(tokens, fa_tokens.unwrap());
    }
}
