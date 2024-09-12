use std::collections::VecDeque;

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
        let mut queue = VecDeque::<IndexedRuleParse<T>>::new();
        for body in &self.rules_by_var[self.start.0] {
            let rule_parse = RuleParse {
                head: self.start,
                body: body.clone(),
                parsed: 0,
            };
            queue.push_back(IndexedRuleParse {
                l: 0,
                r: 0,
                rule_parse,
            });
        }

        while let Some(indexed_rule_parse) = queue.pop_front() {
            let IndexedRuleParse { l, r, rule_parse } = indexed_rule_parse;
            let RuleParse { head, body, parsed } = rule_parse;

            if parsed == body.len() {
                todo!();
                continue;
            }

            match &body[parsed] {
                Symbol::Var(var) => {
                    for body in &self.rules_by_var[var.0] {
                        let rule_parse = RuleParse {
                            head: *var,
                            body: body.clone(),
                            parsed: 0,
                        };
                        queue.push_back(IndexedRuleParse {
                            l: r,
                            r,
                            rule_parse,
                        });
                    }
                }
                Symbol::Term(term) => {
                    todo!()
                }
            }
        }

        todo!()
    }

    fn parse(&self, word: Vec<T>) -> Option<Self::ParseTree> {
        todo!()
    }
}

struct IndexedRuleParse<T: Term> {
    l: usize,
    r: usize,
    rule_parse: RuleParse<T>,
}

struct RuleParse<T: Term> {
    head: Var,
    body: Vec<Symbol<T>>,
    parsed: usize,
}
