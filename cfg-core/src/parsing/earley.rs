use crate::{Cfg, Rule, Symbol, Term, Var};

use super::Parser;

pub struct EarleyParser<T: Term> {
    start: Var,
    rules_by_var: Vec<Vec<Vec<Symbol<T>>>>,
}

impl<T: Term> EarleyParser<T> {
    pub fn of(start: Var, cfg: Cfg<T>) -> Self {
        let mut rules_by_var = vec![vec![]; cfg.vars];
        for Rule { head, body } in cfg.rules {
            rules_by_var[head.0].push(body);
        }

        Self {
            start,
            rules_by_var,
        }
    }
}

impl<T: Term> Parser<Vec<T>> for EarleyParser<T> {
    type ParseTree = ();

    fn test(&self, word: Vec<T>) -> bool {
        let n = word.len();

        let mut states: Vec<Vec<State>> = vec![Vec::new(); n + 1];
        for (i, _) in self.rules_by_var[self.start.0].iter().enumerate() {
            states[0].push(State {
                l: 0,
                head: self.start,
                body_idx: i,
                parsed: 0,
            });
        }

        for r in 0..=n {
            let mut i = 0;
            while i < states[r].len() {
                let State {
                    l,
                    head,
                    body_idx,
                    parsed,
                } = states[r][i];

                let body = &self.rules_by_var[head.0][body_idx];

                if parsed == body.len() {
                    let mut j = 0;
                    while j < states[l].len() {
                        let state = states[l][j];
                        let body = &self.rules_by_var[state.head.0][state.body_idx];
                        if state.parsed == body.len() {
                            j += 1;
                            continue;
                        }
                        let Symbol::Var(var) = &body[state.parsed] else {
                        j += 1;
                            continue;
                        };
                        if *var == head {
                            let state = State {
                                parsed: state.parsed + 1,
                                ..state
                            };
                            if !states[r].contains(&state) {
                                states[r].push(state);
                            }
                        }
                        j += 1;
                    }
                    i += 1;
                    continue;
                }

                match &body[parsed] {
                    Symbol::Var(var) => {
                        for (i, _) in self.rules_by_var[var.0].iter().enumerate() {
                            let state = State {
                                l: r,
                                head: *var,
                                body_idx: i,
                                parsed: 0,
                            };
                            if !states[r].contains(&state) {
                                states[r].push(state);
                            }
                        }
                    }
                    Symbol::Term(term) => {
                        if r < n && word[r] == *term {
                            let state = State {
                                l,
                                head,
                                body_idx,
                                parsed: parsed + 1,
                            };
                            if !states[r + 1].contains(&state) {
                                states[r + 1].push(state);
                            }
                        }
                    }
                }
                i += 1;
            }
        }

        states
            .last()
            .unwrap()
            .into_iter()
            .any(|state| state.l == 0 && state.head == self.start)
    }

    fn parse(&self, _word: Vec<T>) -> Option<Self::ParseTree> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    l: usize,
    head: Var,
    body_idx: usize,
    parsed: usize,
}
