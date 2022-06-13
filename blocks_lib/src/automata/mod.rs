use std::{collections::HashMap, str::Chars};

#[derive(Default, Debug)]
pub struct FiniteAutomata {
    alphabets: Vec<char>,
    states: Vec<String>,
    initial_state: String,
    final_state: String,
    delta: HashMap<(String, char), String>,
}

impl FiniteAutomata {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_alphabets(&mut self, alpha: &[char]) {
        self.alphabets = Vec::from(alpha);
    }

    pub fn add_states(&mut self, states: &[String]) {
        self.states = Vec::from(states);
    }

    pub fn initial_state(&mut self, state: String) {
        self.initial_state = state;
    }

    pub fn final_state(&mut self, state: String) {
        self.final_state = state;
    }

    pub fn add_action(&mut self, state1: String, alpha: &[char], state2: String) {
        for a in alpha {
            let state1 = self.states.iter().position(|s| *s == state1);
            let state2 = self.states.iter().position(|s| *s == state2);
            if state1.is_none() && state2.is_none() {
                return;
            }
            let state1 = self.states[state1.unwrap()].clone();
            let state2 = self.states[state2.unwrap()].clone();
            self.delta.insert((state1, *a), state2);
        }
    }

    pub fn add_actions(&mut self, actions: Vec<(String, &[char], String)>) {
        for action in actions {
            let (state1, alpha, state2) = action;
            self.add_action(state1, alpha, state2);
        }
    }

    pub fn exec(&self, input: Chars) -> Option<bool> {
        let mut curr_state = self.initial_state.clone();
        for a in input {
            let cs = self.delta.get(&(curr_state, a));
            if cs.is_none() {
                return None;
            }
            // dbg!("get here");
            curr_state = cs.unwrap().clone().to_string();
        }

        Some(curr_state == self.final_state)
    }
}

#[cfg(test)]
mod automata_tests {
    use std::vec;

    use super::FiniteAutomata;
    #[test]
    fn basic() {
        let input = "10011101".chars();
        let alphas = ['0', '1'];
        let states = vec!["A".to_owned(), "B".to_owned()];
        let mut fa = FiniteAutomata::new();
        fa.add_alphabets(&alphas);
        fa.add_states(&states);
        fa.initial_state(states[0].clone());
        fa.final_state(states[1].clone());
        fa.add_actions(vec![
            (states[0].clone(), &alphas[0..1], states[0].clone()),
            (states[0].clone(), &alphas[1..2], states[1].clone()),
            (states[1].clone(), &alphas[0..1], states[0].clone()),
            (states[1].clone(), &alphas[1..2], states[1].clone()),
        ]);
        let result = fa.exec(input);
        assert!(result.is_some());
        assert!(result.unwrap());
    }
}
