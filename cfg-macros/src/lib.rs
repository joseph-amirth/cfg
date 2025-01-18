use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprBlock, Ident, Lit, Type};

mod parse;
mod to_tokens;

// TODO: Make the macros more user-friendly somehow.

#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let grammar = parse_macro_input!(input as Grammar);
    quote!(#grammar).into()
}

#[proc_macro]
pub fn interpreted_grammar(input: TokenStream) -> TokenStream {
    let interpreted_grammar = parse_macro_input!(input as InterpretedGrammar);
    quote!(#interpreted_grammar).into()
}

#[derive(Debug)]
struct Grammar {
    rule_sets: Vec<RuleSet>,
}

#[derive(Debug)]
struct RuleSet {
    head: Ident,
    bodies: Vec<Vec<Symbol>>,
}

#[derive(Debug, Clone)]
enum Symbol {
    Var(Ident),
    Term(Lit),
}

#[derive(Debug, Clone)]
struct InterpretedGrammar {
    term_type: Type,
    meaning_type: Type,
    interpreted_rule_sets: Vec<InterpretedRuleSet>,
}

impl Into<Grammar> for InterpretedGrammar {
    fn into(self) -> Grammar {
        let Self {
            interpreted_rule_sets,
            ..
        } = self;
        Grammar {
            rule_sets: interpreted_rule_sets
                .into_iter()
                .map(|interpreted_rule_set| interpreted_rule_set.into())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct InterpretedRuleSet {
    head: Ident,
    bodies: Vec<(Vec<Symbol>, Action)>,
}

impl Into<RuleSet> for InterpretedRuleSet {
    fn into(self) -> RuleSet {
        let Self { head, bodies } = self;
        RuleSet {
            head,
            bodies: bodies.into_iter().map(|(symbols, _)| symbols).collect(),
        }
    }
}

type Action = ExprBlock;
