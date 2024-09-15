use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use crate::{
    cnf::{Cnf, CnfRule},
    Cfg, Term, Var,
};

use super::{ParseTree, Parser};

#[derive(Debug, Clone)]
pub struct CykParser<T: Term> {
    start: Var,
    nullable: Vec<bool>,
    terms_to_vars: HashMap<T, Vec<Var>>,
    binary_rules: Vec<CnfRule<T>>,
}

impl<T: Term> CykParser<T> {
    pub fn of(cfg: Cfg<T>, start: Var) -> Self {
        Self::of_cnf(Cnf::of(cfg), start)
    }

    pub fn of_cnf(
        Cnf {
            nullable,
            rules,
            vars_map,
        }: Cnf<T>,
        start: Var,
    ) -> Self {
        let (terminal_rules, binary_rules) = rules.into_iter().partition(|rule| {
            if let CnfRule::Terminal(_, _) = rule {
                true
            } else {
                false
            }
        });

        let mut terms_to_vars = HashMap::<T, Vec<Var>>::new();
        for terminal_rule in terminal_rules {
            let CnfRule::Terminal(var, term) = terminal_rule else {
                panic!("Expected a terminal rule");
            };
            terms_to_vars
                .entry(term)
                .and_modify(|vars| vars.push(var))
                .or_insert(vec![var]);
        }

        Self {
            start: vars_map[start.0],
            nullable,
            terms_to_vars,
            binary_rules,
        }
    }
}

impl<T: Term> Parser<Vec<T>> for CykParser<T> {
    type TermType = T;

    fn test(&self, input: Vec<T>) -> bool {
        if input.is_empty() {
            return self.nullable[self.start.0];
        }

        let n = input.len();
        let m = self.nullable.len();

        let mut dp = Flat3dVec::filled_with(n, n, m, false);

        for (i, term) in input.into_iter().enumerate() {
            let Some(vars) = self.terms_to_vars.get(&term) else {
                continue;
            };
            for var in vars {
                dp[(i, i, var.0)] = true;
            }
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                for rule in self.binary_rules.iter().cloned() {
                    if let CnfRule::Binary(first, second, third) = rule {
                        for k in i..j {
                            dp[(i, j, first.0)] |= dp[(i, k, second.0)] && dp[(k + 1, j, third.0)];
                        }
                    }
                }
            }
        }

        dp[(0, n - 1, self.start.0)]
    }

    fn parse(&self, word: Vec<T>) -> Option<ParseTree<T>> {
        todo!()
    }
}

#[derive(Debug)]
struct Flat3dVec<T> {
    m: usize,
    n: usize,
    k: usize,
    vec: Vec<T>,
}

impl<T: Clone> Flat3dVec<T> {
    fn filled_with(m: usize, n: usize, k: usize, val: T) -> Self {
        Self {
            m,
            n,
            k,
            vec: vec![val; m * n * k],
        }
    }
}

impl<T> Index<(usize, usize, usize)> for Flat3dVec<T> {
    type Output = T;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        &self.vec[index.0 * self.n * self.k + index.1 * self.k + index.2]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Flat3dVec<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.0 * self.n * self.k + index.1 * self.k + index.2]
    }
}
