use std::collections::HashMap;

use crate::{Cfg, Rule, Symbol, Term, Var};

pub fn eliminate_nonsolitary_terminals<T: Term>(mut cfg: Cfg<T>) -> Cfg<T> {
    let mut term_to_var_map = HashMap::<T, Var>::new();

    for i in 0..cfg.rules.len() {
        if cfg.rules[i].body.len() <= 1 {
            continue;
        }
        for j in 0..cfg.rules[i].body.len() {
            let Symbol::Term(term) = cfg.rules[i].body[j].clone() else {
                continue;
            };
            let mapped_var = match term_to_var_map.get(&term) {
                Some(var) => *var,
                None => {
                    let var = cfg.add_var();
                    cfg.add_rule(Rule::new(var, vec![Symbol::Term(term.clone())]));
                    term_to_var_map.insert(term, var);
                    var
                }
            };
            cfg.rules[i].body[j] = Symbol::Var(mapped_var);
        }
    }

    cfg
}
