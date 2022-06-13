use self::statement::{Statement, Token};

pub mod statement;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Scope {
    GlobalScope,
    BlockScope,
    DeltaScope,
}

use Statement::*;
pub fn parse(code: String) -> Option<Vec<(Statement, Option<Vec<Token>>)>> {
    use Scope::*;
    let mut code_tokens = Vec::new();
    let mut block_scope = GlobalScope;
    let mut i = 0;
    for line in code.lines() {
        let s_type = Statement::match_statement(line);
        if s_type.is_none() {
            return None;
        }
        let s_type = s_type.unwrap();
        match s_type {
            FADef => {
                if block_scope == GlobalScope {
                    block_scope = BlockScope;
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            AlphabetDef => {
                if block_scope == BlockScope && i == 0 {
                    i += 1;
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            StateDef => {
                if block_scope == BlockScope && i == 1 {
                    i += 1;
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            DeltaDef => {
                if block_scope == BlockScope && i == 2 {
                    i += 1;
                    block_scope = DeltaScope;
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            SelfLoop => {
                if block_scope == DeltaScope {
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            Transition => {
                if block_scope == DeltaScope {
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            DeltaDefEnd => {
                if block_scope == DeltaScope && i == 3 {
                    block_scope = BlockScope;
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            FADefEnd => {
                if block_scope == BlockScope && i == 3 {
                    block_scope = GlobalScope;
                    i = 0;
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                } else {
                    return None;
                }
            }
            Out => {
                if block_scope == GlobalScope {
                    let token = Statement::parse_statement(line);
                    if token.is_none() {
                        return None;
                    }
                    code_tokens.push((s_type, token));
                }
            }
            Comment | Empty => {}
        }
    }
    Some(code_tokens)
}

#[cfg(test)]
mod parse_tests {
    use super::{parse, statement::Token::*};
    #[test]
    fn basic() {
        const CODE: &str = include_str!("../../../a.txt");
        let _tokens = Some(vec![
            Some(vec![
                Operator("FA".to_string()),
                Operand("LastOne".to_string()),
                Operator("{".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operator(":=".to_string()),
                Operand("0".to_string()),
                Operand("1".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operator("+".to_string()),
                Operand("(A)".to_string()),
                Operand("{B}".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operator("=>".to_string()),
                Operator("[".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operator("@".to_string()),
                Operand("A".to_string()),
                Operator("=".to_string()),
                Operand("0".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operand("A".to_string()),
                Operator("=".to_string()),
                Operand("1".to_string()),
                Operator("->".to_string()),
                Operand("B".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operand("B".to_string()),
                Operator("=".to_string()),
                Operand("0".to_string()),
                Operator("->".to_string()),
                Operand("A".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![
                Operator("@".to_string()),
                Operand("B".to_string()),
                Operator("=".to_string()),
                Operand("1".to_string()),
                Operator("//".to_string()),
            ]),
            Some(vec![Operator("]".to_string()), Operator("//".to_string())]),
            Some(vec![Operator("}".to_string()), Operator("//".to_string())]),
            Some(vec![
                Operator("FA".to_string()),
                Operand("FirstOne".to_string()),
                Operator("{".to_string()),
            ]),
            Some(vec![
                Operator(":=".to_string()),
                Operand("0".to_string()),
                Operand("1".to_string()),
            ]),
            Some(vec![
                Operator("+".to_string()),
                Operand("(A)".to_string()),
                Operand("{B}".to_string()),
            ]),
            Some(vec![Operator("=>".to_string()), Operator("[".to_string())]),
            Some(vec![
                Operand("A".to_string()),
                Operator("=".to_string()),
                Operand("0".to_string()),
                Operator("->".to_string()),
                Operand("B".to_string()),
            ]),
            Some(vec![
                Operator("@".to_string()),
                Operand("A".to_string()),
                Operator("=".to_string()),
                Operand("1".to_string()),
            ]),
            Some(vec![
                Operand("B".to_string()),
                Operator("=".to_string()),
                Operand("1".to_string()),
                Operator("->".to_string()),
                Operand("A".to_string()),
            ]),
            Some(vec![
                Operator("@".to_string()),
                Operand("B".to_string()),
                Operator("=".to_string()),
                Operand("0".to_string()),
            ]),
            Some(vec![Operator("]".to_string())]),
            Some(vec![Operator("}".to_string())]),
        ]);
        let _p_tokens = parse(CODE.to_owned());
        // assert_eq!(tokens, p_tokens);
    }
}
