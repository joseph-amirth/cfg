use crate::{Cfg, Rule, Symbol, Term};

pub fn eliminate_long_rules<T: Term>(mut cfg: Cfg<T>) -> Cfg<T> {
    for i in 0..cfg.rules.len() {
        while cfg.rules[i].body.len() > 2 {
            let last_symbol = cfg.rules[i].body.pop().unwrap();
            let second_last_symbol = cfg.rules[i].body.pop().unwrap();
            let aux_var = cfg.add_var();
            cfg.add_rule(Rule::new(aux_var, vec![second_last_symbol, last_symbol]));
            cfg.rules[i].body.push(Symbol::Var(aux_var));
        }
    }

    cfg
}
