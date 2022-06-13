use super::{Statement, Token};
use Token::*;

impl Statement {
    pub fn selfloopdef_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let items: Vec<&str> = statement.trim().split_ascii_whitespace().collect();
        let mut tokens = Vec::with_capacity(items.len());
        for (i, item) in items.into_iter().enumerate() {
            match i {
                0 => {
                    let it: Vec<char> = item.chars().collect();
                    if it[0] != '@' {
                        return None;
                    }
                    if !it[1..].into_iter().all(|c| char::is_alphanumeric(*c)) {
                        return None;
                    }
                    tokens.push(Operator(item.get(0..1).unwrap().to_string()));
                    tokens.push(Operand(item.get(1..).unwrap().to_string()));
                }
                1 if item == "=" => tokens.push(Operator(item.to_string())),
                _ => {
                    if item == "//" {
                        tokens.push(Operator(item.to_string()));
                        break;
                    } else if item.chars().all(char::is_alphanumeric) {
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
mod selfloopdef_parse_statement_test {
    use super::super::{Statement, Token::*};
    #[test]
    fn basic() {
        let line = "        @A = 0 1 // ";
        let tokens = vec![
            Operator("@".to_string()),
            Operand("A".to_string()),
            Operator("=".to_string()),
            Operand("0".to_string()),
            Operand("1".to_string()),
            Operator("//".to_string()),
        ];
        let ad_tokens = Statement::parse_statement(line);
        assert!(ad_tokens.is_some());
        assert_eq!(tokens, ad_tokens.unwrap());
    }
}
