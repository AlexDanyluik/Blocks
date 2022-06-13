use super::{Statement, Token};
use Token::*;

impl Statement {
    pub fn out_parse_statement(self, statement: &str) -> Option<Vec<Token>> {
        let items: Vec<&str> = statement.trim().split_ascii_whitespace().collect();
        let mut tokens = Vec::with_capacity(items.len());
        for (i, item) in items.into_iter().enumerate() {
            match i {
                0 if item == ">" => tokens.push(Operator(item.to_string())),
                1 if item.chars().all(char::is_alphabetic) => {
                    tokens.push(Operand(item.to_string()))
                }
                2 if is_string(item) => tokens.push(Operand(from_string(item.to_string()))),
                _ => {
                    if item == "//" {
                        tokens.push(Operator(item.to_string()));
                        break;
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(tokens)
    }
}

fn is_string(item: &str) -> bool {
    let r = item.starts_with('"')
        && item.ends_with('"')
        && item[1..item.len() - 1].chars().all(char::is_alphanumeric);
    r
}

fn from_string(mut item: String) -> String {
    item.remove(0);
    item.remove(item.len() - 1);
    item
}
