use std::collections::VecDeque;

use crate::{Cfg, Rule, Symbol, Term, Var};

use super::weak_cnf::{WeakCnf, WeakCnfRule};

pub fn eliminate_epsilon_rules<T: Term>(cfg: Cfg<T>) -> WeakCnf<T> {
    let nullable = get_nullable_nonterminals(&cfg);

    let mut rules = Vec::<WeakCnfRule<T>>::new();

    for rule in cfg.rules {
        let Rule { head, mut body } = rule;
        if body.len() == 1 {
            match body.pop().unwrap() {
                Symbol::Term(term) => {
                    rules.push(WeakCnfRule::Terminal(head, term));
                }
                Symbol::Var(var) => {
                    rules.push(WeakCnfRule::Unit(head, var));
                }
            }
        } else if body.len() == 2 {
            let Symbol::Var(second) = body.pop().unwrap() else {
                panic!("Expected body with length 2 to only have variables as symbols");
            };
            let Symbol::Var(first) = body.pop().unwrap() else {
                panic!("Expected body with length 2 to only have variables as symbols");
            };
            rules.push(WeakCnfRule::Binary(head, first, second));
            if nullable[first.0] {
                rules.push(WeakCnfRule::Unit(head, second));
            }
            if nullable[second.0] {
                rules.push(WeakCnfRule::Unit(head, first));
            }
        } else {
            panic!("Expected body to have length 1 or 2");
        }
    }

    WeakCnf { nullable, rules }
}

fn get_nullable_nonterminals<T: Term>(cfg: &Cfg<T>) -> Vec<bool> {
    let mut nullable = vec![false; cfg.vars];
    let mut queue = VecDeque::<Var>::new();

    let mut dependant_rules: Vec<Vec<usize>> = vec![vec![]; cfg.vars];

    for (index, rule) in cfg.rules.iter().enumerate() {
        if rule.body.is_empty() {
            nullable[rule.head.0] = true;
            queue.push_back(rule.head);
        } else {
            for symbol in rule.body.iter() {
                if let Symbol::Var(var) = symbol {
                    dependant_rules[var.0].push(index);
                }
            }
        }
    }

    while let Some(front) = queue.pop_front() {
        for index in dependant_rules[front.0].iter() {
            let rule = &cfg.rules[*index];
            let mut is_body_nullable = true;

            for symbol in rule.body.iter() {
                if let Symbol::Var(var) = symbol {
                    is_body_nullable &= nullable[var.0];
                } else {
                    is_body_nullable = false;
                }
            }

            if is_body_nullable && !nullable[rule.head.0] {
                nullable[rule.head.0] = true;
                queue.push_back(rule.head);
            }
        }
    }

    nullable
}
