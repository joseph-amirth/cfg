mod eliminate_epsilon_rules;
mod eliminate_long_rules;
mod eliminate_nonsolitary_terminals;
mod eliminate_unit_rules;
mod weak_cnf;

use crate::{
    cnf::{
        eliminate_epsilon_rules::eliminate_epsilon_rules,
        eliminate_long_rules::eliminate_long_rules,
        eliminate_nonsolitary_terminals::eliminate_nonsolitary_terminals,
        eliminate_unit_rules::eliminate_unit_rules,
    },
    Cfg, Term, Var,
};

#[derive(Debug, Clone)]
pub struct Cnf<T: Term> {
    pub(crate) nullable: Vec<bool>,
    pub(crate) rules: Vec<CnfRule<T>>,
    pub(crate) vars_map: Vec<Var>,
}

impl<T: Term> Cnf<T> {
    pub fn of(cfg: Cfg<T>) -> Self {
        let cfg = eliminate_nonsolitary_terminals(cfg);
        let cfg = eliminate_long_rules(cfg);
        let weak_cnf = eliminate_epsilon_rules(cfg);
        let cnf = eliminate_unit_rules(weak_cnf);

        cnf
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CnfRule<T: Term> {
    Terminal(Var, T),      // A => a
    Binary(Var, Var, Var), // A => BC
}
