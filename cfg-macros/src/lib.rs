use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprBlock, Ident, Lit, Type};

mod parse;
mod to_tokens;

#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let grammar = parse_macro_input!(input as Grammar);
    quote!(#grammar).into()
}

#[proc_macro]
pub fn grammar_with_actions(input: TokenStream) -> TokenStream {
    let grammar_with_actions = parse_macro_input!(input as GrammarWithActions);
    quote!(#grammar_with_actions).into()
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

#[derive(Debug)]
enum Symbol {
    Var(Ident),
    Term(Lit),
}

#[derive(Debug)]
struct GrammarWithActions {
    rule_sets_with_actions: Vec<RuleSetWithActions>,
}

#[derive(Debug)]
struct RuleSetWithActions {
    head: Ident,
    head_type: Type,
    bodies: Vec<(Vec<Symbol>, Action)>,
}

type Action = Option<ExprBlock>;
