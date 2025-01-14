use std::rc::Rc;

use crate::{Cfg, Rule, Symbol, Term, Var};

use super::{ParseTree, ParsedSymbol, Parser};

pub struct EarleyParser<T: Term> {
    start_var: Var,
    var_names: Vec<Rc<str>>,
    rules: Vec<Rule<T>>,
    rules_by_var: Vec<Vec<usize>>,
}

impl<T: Term> EarleyParser<T> {
    pub fn of(cfg: Cfg<T>) -> Self {
        let mut rules_by_var = vec![vec![]; cfg.n_vars()];

        let Cfg {
            start_var,
            var_names,
            rules,
        } = cfg;

        for (i, rule) in rules.iter().enumerate() {
            rules_by_var[rule.head.0].push(i);
        }

        Self {
            start_var,
            var_names,
            rules,
            rules_by_var,
        }
    }

    fn attempt_parse(&self, word: &Vec<T>) -> (Vec<Vec<State>>, Vec<Vec<Parent>>) {
        let n = word.len();

        let mut states: Vec<Vec<State>> = vec![Vec::new(); n + 1];
        let mut parents: Vec<Vec<Parent>> = vec![Vec::new(); n + 1];

        for rule_idx in self.rules_by_var[self.start_var.0].iter().cloned() {
            states[0].push(State {
                l: 0,
                rule_idx,
                parsed: 0,
            });
            parents[0].push(Parent::None);
        }

        for r in 0..=n {
            let mut i = 0;
            while i < states[r].len() {
                let State {
                    l,
                    rule_idx,
                    parsed,
                } = states[r][i];

                let head = self.rules[rule_idx].head;
                let body = &self.rules[rule_idx].body;

                if parsed == body.len() {
                    let mut j = 0;
                    while j < states[l].len() {
                        let state = states[l][j];
                        let body = &self.rules[state.rule_idx].body;
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
                        for rule_idx in self.rules_by_var[var.0].iter().cloned() {
                            let state = State {
                                l: r,
                                rule_idx,
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
                                rule_idx,
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
            self.rules[state.rule_idx].head == self.start_var
                && state.l == 0
                && state.parsed == self.rules[state.rule_idx].body.len()
        })
    }

    fn parse(&self, word: Vec<T>) -> Option<ParseTree<T>> {
        let n = word.len();
        let (states, parents) = self.attempt_parse(&word);

        let Some(final_state_pos) = states[n].iter().position(|state| {
            self.rules[state.rule_idx].head == self.start_var
                && state.l == 0
                && state.parsed == self.rules[state.rule_idx].body.len()
        }) else {
            return None;
        };

        Some(
            ParseTreeBuilder {
                parser: &self,
                word,
                states,
                parents,
            }
            .build(n, final_state_pos),
        )
    }
}

struct ParseTreeBuilder<'a, T: Term> {
    parser: &'a EarleyParser<T>,
    word: Vec<T>,
    states: Vec<Vec<State>>,
    parents: Vec<Vec<Parent>>,
}

impl<T: Term> ParseTreeBuilder<'_, T> {
    pub fn build(&self, r: usize, i: usize) -> ParseTree<T> {
        match &self.parents[r][i] {
            Parent::None => {
                let rule_idx = self.states[r][i].rule_idx;
                let root_var = self.parser.rules[rule_idx].head;
                ParseTree {
                    root_var,
                    root_var_name: self.parser.var_names[root_var.0].to_owned(),
                    rule_idx,
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
    rule_idx: usize,
    parsed: usize,
}

#[derive(Debug, Clone, Copy)]
enum Parent {
    None,
    Term(usize),
    Var(usize, usize),
}
