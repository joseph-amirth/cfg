use std::{fmt::Debug, hash::Hash, rc::Rc};

use rand::{rngs::ThreadRng, seq::IteratorRandom};

pub mod cnf;
pub mod interpret;
pub mod parse;

#[derive(Debug, Clone)]
pub struct Cfg<T: Term> {
    start_var: Var,
    var_names: Vec<Rc<str>>,
    rules: Vec<Rule<T>>,
}

impl<T: Term> Cfg<T> {
    pub fn builder() -> CfgBuilder<T> {
        CfgBuilder {
            var_names: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn to_builder(self) -> CfgBuilder<T> {
        let Self {
            var_names, rules, ..
        } = self;
        CfgBuilder { var_names, rules }
    }

    pub fn n_vars(&self) -> usize {
        self.var_names.len()
    }

    pub fn rules(&self, var: Var) -> impl Iterator<Item = &Rule<T>> {
        self.rules.iter().filter(move |rule| rule.head == var)
    }

    /// Generates a random word in the language defined by the given variable.
    ///
    /// **Warning**: The distribution of words is not guaranteed. It is not even
    /// guaranteed that this function terminates.
    pub fn random_word(&self) -> Vec<T> {
        self.random_word_impl(self.start_var, &mut rand::thread_rng())
    }

    fn random_word_impl(&self, var: Var, rng: &mut ThreadRng) -> Vec<T> {
        let rules = self.rules(var);
        let random_rule = rules.choose(rng).unwrap();

        let mut word = Vec::new();
        for symbol in &random_rule.body {
            match symbol {
                Symbol::Term(term) => {
                    word.push(term.to_owned());
                }
                Symbol::Var(var) => {
                    word.extend(self.random_word_impl(*var, rng).into_iter());
                }
            }
        }

        word
    }
}

pub struct CfgBuilder<T: Term> {
    var_names: Vec<Rc<str>>,
    rules: Vec<Rule<T>>,
}

impl<T: Term> CfgBuilder<T> {
    pub fn add_var(&mut self, name: Rc<str>) -> Var {
        let var = Var(self.var_names.len());
        self.var_names.push(name);
        var
    }

    pub fn add_rule(&mut self, rule: Rule<T>) -> &mut Self {
        self.rules.push(rule);
        self
    }

    pub fn add_rules(&mut self, rules: impl IntoIterator<Item = Rule<T>>) -> &mut Self {
        for rule in rules {
            self.add_rule(rule);
        }
        self
    }

    pub fn build(self, start_var: Var) -> Cfg<T> {
        let Self { var_names, rules } = self;
        Cfg {
            start_var,
            var_names,
            rules,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule<T: Term> {
    pub(crate) head: Var,
    pub(crate) body: Vec<Symbol<T>>,
}

impl<T: Term> Rule<T> {
    pub fn new(head: Var, body: Vec<Symbol<T>>) -> Self {
        Self { head, body }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol<T: Term> {
    Var(Var),
    Term(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var(pub(crate) usize);

pub trait Term: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash {}

impl<T: Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash> Term for T {}
