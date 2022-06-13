use core::panic;
use std::collections::HashMap;

use crate::{
    automata::FiniteAutomata,
    parser::{
        parse,
        statement::{Statement, Token},
        Scope,
    },
};

pub struct Engine {
    blocks: HashMap<String, FiniteAutomata>,
    output: Vec<String>,
}

impl Engine {
    pub fn from_str(s: String) -> Self {
        let mut engine = Engine {
            blocks: HashMap::new(),
            output: Vec::new(),
        };
        let statements = parse(s).unwrap();
        let mut scope = Scope::GlobalScope;
        let mut curr_block = None;
        for (statement, token) in statements.into_iter() {
            let mut tokens = token.unwrap();
            match scope {
                Scope::GlobalScope if curr_block.is_none() => {
                    if statement == Statement::FADef {
                        scope = Scope::BlockScope;
                        let fa = FiniteAutomata::new();
                        if let Token::Operand(name) = &tokens[1] {
                            curr_block = Some(name.to_owned());
                            engine.blocks.insert(name.to_owned(), fa);
                        }
                    } else if statement == Statement::Out {
                        if let Token::Operand(name) = &tokens[1] {
                            if engine.blocks.contains_key(name) {
                                if let Token::Operand(arg) = &tokens[2] {
                                    let result = engine.block(name).exec(arg.chars());
                                    if result.is_none() {
                                        panic!("Invalid Output Syntax");
                                    }
                                    engine.output.push(format!("{}", result.unwrap()));
                                }
                            }
                        } else {
                            panic!("Invalid Output Syntax 1");
                        }
                    } else if statement == Statement::FADefEnd {
                        scope = Scope::GlobalScope;
                        curr_block = None;
                    } else {
                        panic!("Not in GlobalScope");
                    }
                }
                Scope::BlockScope if curr_block.is_some() => {
                    if statement == Statement::AlphabetDef {
                        let mut alphas = Vec::new();
                        for token in &tokens[1..] {
                            if let Token::Operator(_) = token {
                                break;
                            }
                            if let Token::Operand(c) = token {
                                alphas.push(c.chars().nth(0).unwrap());
                            }
                        }
                        if let Some(name) = &curr_block {
                            let block = engine.blocks.get_mut(name).unwrap();
                            block.add_alphabets(&alphas);
                        }
                    } else if statement == Statement::StateDef {
                        let mut states = Vec::new();
                        let mut init_state = String::new();
                        let mut final_state = String::new();
                        let mut is_init = false;
                        let mut is_final = false;
                        tokens.remove(0);
                        for token in tokens.into_iter() {
                            if let Token::Operator(_) = token {
                                break;
                            }
                            if let Token::Operand(mut c) = token {
                                if is_final_state(&c) {
                                    if !is_final {
                                        c = state_from_final_state(c);
                                        is_final = true;
                                    } else {
                                        panic!("Cannot have more than one final state");
                                    }
                                    if is_initial_state(&c) {
                                        if !is_init {
                                            c = state_from_initial_state(c);
                                            is_init = true;
                                        } else {
                                            panic!("Cannot have more than one initial state");
                                        }
                                        init_state = c.clone();
                                    }
                                    final_state = c.clone();
                                }
                                if is_initial_state(&c) {
                                    if !is_init {
                                        c = state_from_initial_state(c);
                                        is_init = true;
                                    } else {
                                        panic!("Cannot have more than one final state");
                                    }
                                    if is_final_state(&c) {
                                        if !is_final {
                                            c = state_from_final_state(c);
                                            is_final = true;
                                        } else {
                                            panic!("Cannot have more than one initial state");
                                        }
                                        final_state = c.clone();
                                    }
                                    init_state = c.clone();
                                }

                                states.push(c);
                            }
                        }
                        if let Some(name) = &curr_block {
                            let block = engine.blocks.get_mut(name).unwrap();
                            block.add_states(&states);
                            if is_init && is_final {
                                block.initial_state(init_state);
                                block.final_state(final_state);
                            } else {
                                panic!("No initial or final state found!");
                            }
                        }
                    } else if statement == Statement::DeltaDef {
                        scope = Scope::DeltaScope;
                    } else if statement == Statement::FADefEnd {
                        scope = Scope::GlobalScope;
                        curr_block = None;
                    } else {
                        panic!("Not in Block");
                    }
                }
                Scope::DeltaScope if curr_block.is_some() => {
                    if statement == Statement::Transition {
                        let mut i = 1;
                        let mut state1 = String::new();
                        let mut state2 = String::new();
                        let mut alphas = Vec::new();
                        for token in tokens.into_iter() {
                            match token {
                                Token::Operand(c) if i == 1 => {
                                    i += 1;
                                    state1 = c;
                                }
                                Token::Operator(_) if i == 2 => i += 1,
                                Token::Operand(c) if i == 3 => {
                                    alphas.push(c.chars().nth(0).unwrap())
                                }
                                Token::Operator(_) if i == 3 => i += 1,
                                Token::Operand(c) if i == 4 => {
                                    state2 = c.to_owned();
                                    i += 1;
                                }
                                _ if i > 4 => {
                                    break;
                                }
                                _ => panic!("Not in Delta"),
                            }
                        }
                        if let Some(name) = &curr_block {
                            let block = engine.blocks.get_mut(name).unwrap();
                            block.add_action(state1, &alphas, state2);
                        }
                    } else if statement == Statement::SelfLoop {
                        let mut i = 1;
                        let mut state = String::new();
                        let mut alphas = Vec::new();
                        tokens.remove(0);
                        for token in tokens.into_iter() {
                            match token {
                                Token::Operand(c) if i == 1 => {
                                    i += 1;
                                    state = c;
                                }
                                Token::Operator(_) if i == 2 => i += 1,
                                Token::Operand(c) if i == 3 => {
                                    alphas.push(c.chars().nth(0).unwrap());
                                }
                                Token::Operator(_) if i >= 2 => {
                                    break;
                                }
                                _ => panic!("Not in Delta"),
                            }
                            if let Some(name) = &curr_block {
                                let block = engine.blocks.get_mut(name).unwrap();
                                block.add_action(state.clone(), &alphas, state.clone());
                            }
                        }
                    } else if statement == Statement::DeltaDefEnd {
                        scope = Scope::BlockScope;
                    } else {
                        panic!("Not in Delta");
                    }
                }
                _ => panic!("Token Error"),
            }
        }
        engine
    }
    pub fn block(&self, name: &str) -> &FiniteAutomata {
        self.blocks.get(name).unwrap()
    }

    pub fn output(&self) -> &[String] {
        &self.output
    }
}

fn is_initial_state(c: &str) -> bool {
    c.starts_with("(") && c.ends_with(")")
}

fn is_final_state(c: &str) -> bool {
    c.starts_with("{") && c.ends_with("}")
}

fn state_from_initial_state(mut c: String) -> String {
    c.remove(0);
    c.remove(c.len() - 1);
    c
}

fn state_from_final_state(mut c: String) -> String {
    c.remove(0);
    c.remove(c.len() - 1);
    c
}
