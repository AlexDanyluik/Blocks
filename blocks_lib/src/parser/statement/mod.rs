#![allow(dead_code)]

mod alphadef_statement;
mod comment_statement;
mod deltadef_statement;
mod deltadefend_statement;
mod empty_statement;
mod fadef_statement;
mod fadefend_statement;
mod out_statement;
mod selfloopdef_statement;
mod statedef_statement;
mod transdef_statement;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Statement {
    FADef,
    AlphabetDef,
    StateDef,
    DeltaDef,
    SelfLoop,
    Transition,
    DeltaDefEnd,
    FADefEnd,
    Out,
    Comment,
    Empty,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Operand(String),
    Operator(String),
}

impl Statement {
    pub fn match_statement(statement: &str) -> Option<Statement> {
        let statement = statement.trim();
        let statement = statement.as_bytes();
        let mut i = 0;
        let i = loop {
            if i == statement.len() {
                return Some(Statement::Empty);
            }
            let b = statement[i];
            //ignore all the starting whitespace
            if b != b' ' && b != b'\t' {
                break i;
            }
            i += 1;
        };
        match statement[i] {
            b'F' if statement.len() > 1
                && statement[i + 1] == b'A'
                && statement.len() > 2
                && (statement[i + 2] == b' ' || statement[i + 2] == b'\t') =>
            {
                Some(Statement::FADef)
            }
            b':' if statement.len() > 1
                && statement[i + 1] == b'='
                && statement.len() > 2
                && (statement[i + 2] == b' ' || statement[i + 2] == b'\t') =>
            {
                Some(Statement::AlphabetDef)
            }
            b'>' if statement.len() > 1 && statement[i + 1] == b' '
                || statement[i + 1] == b'\t' =>
            {
                Some(Statement::Out)
            }
            b'+' if statement.len() > 1 && statement[i + 1] == b' '
                || statement[i + 1] == b'\t' =>
            {
                Some(Statement::StateDef)
            }
            b'=' if statement.len() > 1
                && statement[i + 1] == b'>'
                && statement.len() > 2
                && (statement[i + 2] == b' ' || statement[i + 2] == b'\t') =>
            {
                Some(Statement::DeltaDef)
            }
            b'@' if statement.len() > 1
                && statement[i + 1].is_ascii_alphanumeric()
                && (statement[i + 1] != b' ' || statement[i + 1] != b'\t') =>
            {
                Some(Statement::SelfLoop)
            }
            b if b.is_ascii_alphanumeric() => Some(Statement::Transition),
            b']' => {
                if statement.len() > 1 && (statement[i + 1] != b' ' || statement[i + 1] != b'\t') {
                    return None;
                }
                Some(Statement::DeltaDefEnd)
            }
            b'}' => {
                if statement.len() > 1 && (statement[i + 1] != b' ' || statement[i + 1] != b'\t') {
                    return None;
                }
                Some(Statement::FADefEnd)
            }
            b'/' if statement.len() > 1 && statement[i + 1] == b'/' => Some(Statement::Comment),
            _ => None,
        }
    }
    pub fn parse_statement(statement: &str) -> Option<Vec<Token>> {
        let s = Self::match_statement(statement);
        if s.is_none() {
            return None;
        }
        let s = s.unwrap();
        match s {
            Statement::FADef => s.fadef_parse_statement(statement),
            Statement::AlphabetDef => s.alphadef_parse_statement(statement),
            Statement::StateDef => s.statedef_parse_statement(statement),
            Statement::DeltaDef => s.deltadef_parse_statement(statement),
            Statement::SelfLoop => s.selfloopdef_parse_statement(statement),
            Statement::Transition => s.transdef_parse_statement(statement),
            Statement::DeltaDefEnd => s.deltadefend_parse_statement(statement),
            Statement::FADefEnd => s.fadefend_parse_statement(statement),
            Statement::Out => s.out_parse_statement(statement),
            Statement::Comment => s.comment_parse_statement(statement),
            Statement::Empty => s.empty_parse_statement(statement),
        }
    }
}

#[cfg(test)]
mod match_statement_tests {
    use super::Statement;
    #[test]
    fn basic() {
        let code = "
// first one

FA FirstOne {
    := 0 1
    + (A) {B}
    => [
        A = 0 -> B
        @A = 1
        B = 1 -> A
        @B = 0
    ]
} 
> FirstOne \"1001\"";
        const STATEMENTS: [Statement; 14] = [
            Statement::Empty,
            Statement::Comment,
            Statement::Empty,
            Statement::FADef,
            Statement::AlphabetDef,
            Statement::StateDef,
            Statement::DeltaDef,
            Statement::Transition,
            Statement::SelfLoop,
            Statement::Transition,
            Statement::SelfLoop,
            Statement::DeltaDefEnd,
            Statement::FADefEnd,
            Statement::Out,
        ];
        let mut i = 0;
        let lines = code.lines();
        for line in lines {
            let statement = Statement::match_statement(line);
            assert_eq!(true, statement.is_some());
            let statement = statement.unwrap();
            assert_eq!(STATEMENTS[i], statement);
            i += 1;
        }
    }
}
#[cfg(test)]
mod parse_statement_tests {
    use crate::parser::statement::Token::{self, *};

    use super::Statement;
    #[test]
    fn basic() {
        let code = "
// first one
FA FirstOne {
    := 0 1 // hello
    + (A) {B}
    => [
        A = 0 -> B
        @A = 1
        B = 1 -> A
        @B = 0
    ]
} ";
        let tokens: Vec<Vec<Token>> = vec![
            vec![Operator("".to_string())],
            vec![Operator("//".to_string())],
            vec![
                Operator("FA".to_string()),
                Operand("FirstOne".to_string()),
                Operator("{".to_string()),
            ],
            vec![
                Operator(":=".to_string()),
                Operand("0".to_string()),
                Operand("1".to_string()),
                Operator("//".to_string()),
            ],
            vec![
                Operator("+".to_string()),
                Operand("(A)".to_string()),
                Operand("{B}".to_string()),
            ],
            vec![Operator("=>".to_string()), Operator("[".to_string())],
            vec![
                Operand("A".to_string()),
                Operator("=".to_string()),
                Operand("0".to_string()),
                Operator("->".to_string()),
                Operand("B".to_string()),
            ],
            vec![
                Operator("@".to_string()),
                Operand("A".to_string()),
                Operator("=".to_string()),
                Operand("1".to_string()),
            ],
            vec![
                Operand("B".to_string()),
                Operator("=".to_string()),
                Operand("1".to_string()),
                Operator("->".to_string()),
                Operand("A".to_string()),
            ],
            vec![
                Operator("@".to_string()),
                Operand("B".to_string()),
                Operator("=".to_string()),
                Operand("0".to_string()),
            ],
            vec![Operator("]".to_string())],
            vec![Operator("}".to_string())],
        ];
        let mut i = 0;
        let lines = code.lines();
        for line in lines {
            // dbg!(Statement::parse_statement(line));
            let p_tokens = Statement::parse_statement(line);
            assert!(p_tokens.is_some());
            assert_eq!(tokens[i], p_tokens.unwrap());
            i += 1;
        }
    }
}
