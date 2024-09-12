use crate::{Term, Var};

#[derive(Debug, Clone, Copy)]
pub enum WeakCnfRule<T: Term> {
    Terminal(Var, T),      // A => a
    Unit(Var, Var),        // A => B
    Binary(Var, Var, Var), // A => BC
}

#[derive(Debug, Clone, Default)]
pub struct WeakCnf<T: Term> {
    pub nullable: Vec<bool>,
    pub rules: Vec<WeakCnfRule<T>>,
}
