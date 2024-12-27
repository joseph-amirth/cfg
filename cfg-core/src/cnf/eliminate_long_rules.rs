use crate::{Cfg, Rule, Symbol, Term};

pub fn eliminate_long_rules<T: Term>(cfg: Cfg<T>) -> Cfg<T> {
    let start_var = cfg.start_var;
    let mut cfg_builder = cfg.to_builder();
    for i in 0..cfg_builder.rules.len() {
        while cfg_builder.rules[i].body.len() > 2 {
            let last_symbol = cfg_builder.rules[i].body.pop().unwrap();
            let second_last_symbol = cfg_builder.rules[i].body.pop().unwrap();
            let aux_var = cfg_builder.add_var("<GEN_AUX>".into());
            cfg_builder.add_rule(Rule::new(aux_var, vec![second_last_symbol, last_symbol]));
            cfg_builder.rules[i].body.push(Symbol::Var(aux_var));
        }
    }

    cfg_builder.build(start_var)
}
