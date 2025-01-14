use crate::{Action, InterpretedGrammar, InterpretedRuleSet};

use super::{Grammar, RuleSet, Symbol};
use syn::{parse::Parse, ExprBlock, Ident, Token};

impl Parse for Grammar {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rule_sets = Vec::new();
        while let Ok(rule_set) = input.parse() {
            rule_sets.push(rule_set);
        }
        Ok(Grammar { rule_sets })
    }
}

impl Parse for RuleSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let head = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;

        let mut bodies: Vec<Vec<Symbol>> = Vec::new();
        let mut current_body: Vec<Symbol> = Vec::new();

        loop {
            if input.peek(Ident) && input.peek2(Token![=>]) {
                break;
            }
            if input.is_empty() {
                break;
            }
            if let Ok(symbol) = input.parse() {
                current_body.push(symbol);
            } else {
                input.parse::<Token![|]>()?;
                bodies.push(current_body);
                current_body = Vec::new();
            }
        }

        bodies.push(current_body);
        Ok(RuleSet { head, bodies })
    }
}

impl Parse for Symbol {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(ident) = input.parse() {
            return Ok(Symbol::Var(ident));
        }
        if let Ok(lit) = input.parse() {
            return Ok(Symbol::Term(lit));
        }
        Err(input.error("Expected an identifier or literal"))
    }
}

impl Parse for InterpretedGrammar {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut interpreted_rule_sets = Vec::new();
        while let Ok(interpreted_rule_set) = input.parse() {
            interpreted_rule_sets.push(interpreted_rule_set);
        }
        Ok(InterpretedGrammar {
            interpreted_rule_sets,
        })
    }
}

impl Parse for InterpretedRuleSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let head = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;

        let mut bodies: Vec<(Vec<Symbol>, Action)> = Vec::new();

        loop {
            let mut current_body: Vec<Symbol> = Vec::new();
            while !(input.peek(Ident) && input.peek2(Token![=>])) {
                if let Ok(symbol) = input.parse() {
                    current_body.push(symbol);
                } else {
                    break;
                }
            }
            let action = input.parse::<ExprBlock>()?;
            bodies.push((current_body, action));

            if input.is_empty() || (input.peek(Ident) && input.peek2(Token![=>])) {
                break;
            }
            input.parse::<Token![|]>()?;
        }

        Ok(InterpretedRuleSet { head, bodies })
    }
}
