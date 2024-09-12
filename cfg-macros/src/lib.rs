use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Lit};

mod parse;
mod to_tokens;

#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let grammar = parse_macro_input!(input as Grammar);
    quote!(#grammar).into()
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
