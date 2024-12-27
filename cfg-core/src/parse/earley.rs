use std::rc::Rc;

use crate::{Cfg, Rule, Symbol, Term, Var};

use super::{ParseTree, ParsedSymbol, Parser};

pub struct EarleyParser<T: Term> {
    start_var: Var,
    var_names: Vec<Rc<str>>,
    rules_by_var: Vec<Vec<Vec<Symbol<T>>>>,
}

impl<T: Term> EarleyParser<T> {
    pub fn of(cfg: Cfg<T>) -> Self {
        let mut rules_by_var = vec![vec![]; cfg.n_vars()];

        let Cfg {
            start_var,
            var_names,
            rules,
        } = cfg;

        for Rule { head, body } in rules {
            rules_by_var[head.0].push(body);
        }

        Self {
            start_var,
            var_names,
            rules_by_var,
        }
    }

    fn attempt_parse(&self, word: &Vec<T>) -> (Vec<Vec<State>>, Vec<Vec<Parent>>) {
        let n = word.len();

        let mut states: Vec<Vec<State>> = vec![Vec::new(); n + 1];
        let mut parents: Vec<Vec<Parent>> = vec![Vec::new(); n + 1];

        for (i, _) in self.rules_by_var[self.start_var.0].iter().enumerate() {
            states[0].push(State {
                l: 0,
                head: self.start_var,
                body_idx: i,
                parsed: 0,
            });
            parents[0].push(Parent::None);
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
                                parents[r].push(Parent::Var(i, j));
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
                                parents[r].push(Parent::None);
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
                                parents[r + 1].push(Parent::Term(i));
                            }
                        }
                    }
                }
                i += 1;
            }
        }

        (states, parents)
    }
}

impl<T: Term> Parser<Vec<T>> for EarleyParser<T> {
    type TermType = T;

    fn test(&self, word: Vec<T>) -> bool {
        let n = word.len();
        let (states, _) = self.attempt_parse(&word);

        states[n].iter().any(|state| {
            state.head == self.start_var
                && state.l == 0
                && state.parsed == self.rules_by_var[self.start_var.0][state.body_idx].len()
        })
    }

    fn parse(&self, word: Vec<T>) -> Option<ParseTree<T>> {
        let n = word.len();
        let (states, parents) = self.attempt_parse(&word);

        let Some(final_state_pos) = states[n].iter().position(|state| {
            state.head == self.start_var
                && state.l == 0
                && state.parsed == self.rules_by_var[self.start_var.0][state.body_idx].len()
        }) else {
            return None;
        };

        Some(
            ParseTreeBuilder {
                var_names: &self.var_names,
                word,
                states,
                parents,
            }
            .build(n, final_state_pos),
        )
    }
}

struct ParseTreeBuilder<'a, T: Term> {
    var_names: &'a Vec<Rc<str>>,
    word: Vec<T>,
    states: Vec<Vec<State>>,
    parents: Vec<Vec<Parent>>,
}

impl<T: Term> ParseTreeBuilder<'_, T> {
    pub fn build(&self, r: usize, i: usize) -> ParseTree<T> {
        match &self.parents[r][i] {
            Parent::None => {
                let root_var = self.states[r][i].head;
                ParseTree {
                    root_var,
                    root_var_name: self.var_names[root_var.0].to_owned(),
                    children: Vec::new(),
                }
            }
            Parent::Term(k) => {
                let mut parse_tree = self.build(r - 1, *k);
                parse_tree
                    .children
                    .push(ParsedSymbol::Term(self.word[r - 1].clone()));
                parse_tree
            }
            Parent::Var(k, l) => {
                let mut parse_tree = self.build(self.states[r][*k].l, *l);
                let another_parse_tree = self.build(r, *k);
                parse_tree
                    .children
                    .push(ParsedSymbol::Var(Box::new(another_parse_tree)));
                parse_tree
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    l: usize,
    head: Var,
    body_idx: usize,
    parsed: usize,
}

#[derive(Debug, Clone, Copy)]
enum Parent {
    None,
    Term(usize),
    Var(usize, usize),
}
