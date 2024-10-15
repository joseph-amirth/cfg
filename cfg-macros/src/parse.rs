use std::collections::HashMap;

use crate::{Action, GrammarWithActions, RuleSetWithActions};

use super::{Grammar, RuleSet, Symbol};
use quote::quote;
use syn::{parse::Parse, spanned::Spanned, ExprBlock, Ident, Token, Type};

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

impl Parse for GrammarWithActions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rule_sets_with_actions = Vec::new();
        while let Ok(rule_set_with_actions) = input.parse() {
            rule_sets_with_actions.push(rule_set_with_actions);
        }
        let grammar_with_actions = GrammarWithActions {
            rule_sets_with_actions,
        };
        grammar_with_actions.check_var_type_validity()?;
        Ok(grammar_with_actions)
    }
}

impl GrammarWithActions {
    /// Checks that the same var isn't declared to have multiple types.
    fn check_var_type_validity(&self) -> syn::Result<()> {
        let mut var_type_map = HashMap::new();
        for rule_set_with_actions in &self.rule_sets_with_actions {
            let var_type = var_type_map
                .entry(rule_set_with_actions.head.clone())
                .or_insert(rule_set_with_actions.head_type.clone());

            if *var_type != rule_set_with_actions.head_type {
                let head = &rule_set_with_actions.head;
                let conflicting_type = &rule_set_with_actions.head_type;
                return Err(syn::Error::new(
                    rule_set_with_actions.head_type.span(),
                    format!(
                        "Type of '{}' is declared as '{}', but it has previously been declared as '{}'",
                        quote!(#head).to_string(),
                        quote!(#conflicting_type).to_string(),
                        quote!(#var_type).to_string(),
                    ),
                ));
            }
        }
        Ok(())
    }
}

impl Parse for RuleSetWithActions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let head = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let head_type = input.parse::<Type>()?;
        input.parse::<Token![=>]>()?;

        let mut bodies: Vec<(Vec<Symbol>, Action)> = Vec::new();

        loop {
            let mut current_body: Vec<Symbol> = Vec::new();
            while !input.is_empty() && !(input.peek(Ident) && input.peek2(Token![:])) {
                if let Ok(symbol) = input.parse() {
                    current_body.push(symbol);
                } else {
                    break;
                }
            }
            let action = input.parse::<ExprBlock>().ok();
            bodies.push((current_body, action));

            if input.is_empty() || (input.peek(Ident) && input.peek2(Token![:])) {
                break;
            }
            input.parse::<Token![|]>()?;
        }

        Ok(RuleSetWithActions {
            head,
            head_type,
            bodies,
        })
    }
}
