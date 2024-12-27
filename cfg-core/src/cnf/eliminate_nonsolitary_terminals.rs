use std::collections::HashMap;

use crate::{Cfg, Rule, Symbol, Term, Var};

pub fn eliminate_nonsolitary_terminals<T: Term>(cfg: Cfg<T>) -> Cfg<T> {
    let start_var = cfg.start_var;
    let mut cfg_builder = cfg.to_builder();
    let mut term_to_var_map = HashMap::<T, Var>::new();

    for i in 0..cfg_builder.rules.len() {
        if cfg_builder.rules[i].body.len() <= 1 {
            continue;
        }
        for j in 0..cfg_builder.rules[i].body.len() {
            let Symbol::Term(term) = cfg_builder.rules[i].body[j].clone() else {
                continue;
            };
            let mapped_var = match term_to_var_map.get(&term) {
                Some(var) => *var,
                None => {
                    let var = cfg_builder.add_var("<GEN_TERM>".into());
                    cfg_builder.add_rule(Rule::new(var, vec![Symbol::Term(term.clone())]));
                    term_to_var_map.insert(term, var);
                    var
                }
            };
            cfg_builder.rules[i].body[j] = Symbol::Var(mapped_var);
        }
    }

    cfg_builder.build(start_var)
}
