use crate::{Grammar, InterpretedGrammar, InterpretedRuleSet, RuleSet, Symbol};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::iter::Iterator;
use syn::LitStr;

impl ToTokens for Grammar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut vars = Vec::new();
        for rule_set in &self.rule_sets {
            vars.push(&rule_set.head);
        }

        // TODO: Add support for specifying a var besides the head of the first rule set as the
        // start var.
        let start_var = vars.first().unwrap().to_owned();

        vars.sort();
        vars.dedup();

        let var_lits = vars
            .iter()
            .cloned()
            .map(|ident| LitStr::new(ident.to_string().as_str(), ident.span()))
            .collect::<Vec<_>>();

        let rule_sets = &self.rule_sets;

        tokens.append_all(quote!({
            let mut cfg_builder = cfg::Cfg::builder();
            #( let #vars = cfg_builder.add_var(#var_lits.into()); )*
            #( #rule_sets )*
            cfg_builder.build(#start_var)
        }));
    }
}

impl ToTokens for RuleSet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let head = &self.head;
        let bodies = &self.bodies;

        tokens.append_all(quote!(
            #({
                cfg_builder.add_rule(
                    cfg::Rule::new(
                        #head,
                        vec![#( #bodies ),*]
                    )
                );
            })*
        ));
    }
}

impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let symbol_tokens = match self {
            Symbol::Var(ident) => {
                quote!(cfg::Symbol::Var(#ident))
            }
            Symbol::Term(lit) => {
                quote!(cfg::Symbol::Term(#lit))
            }
        };
        tokens.append_all(symbol_tokens);
    }
}

impl ToTokens for InterpretedGrammar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let grammar: Grammar = self.to_owned().into();
        let interpreted_rule_sets = &self.interpreted_rule_sets;
        tokens.append_all(quote!({
            use cfg::interpret::InterpretedSymbol;
            let mut rules = Vec::<fn(Vec<InterpretedSymbol<char, i32>>) -> i32>::new();
            #( #interpreted_rule_sets )*
            (#grammar, cfg::interpret::Interpreter::new(rules))
        }));
    }
}

impl ToTokens for InterpretedRuleSet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let head = &self.head;
        for (i, (symbols, action)) in self.bodies.iter().enumerate() {
            let fun_name = format_ident!("interpret_{}_{}", head, i);
            let bindings = symbols.iter().enumerate().map(|(i, symbol)| {
                let symbol_var_name = format_ident!("_{}", i + 1);
                match symbol {
                    Symbol::Term(_) => {
                        quote!(let InterpretedSymbol::Term(#symbol_var_name) = symbols_iter.next().unwrap() else {
                            panic!("Unexpected error");
                        };)
                    }
                    Symbol::Var(_) => {
                        quote!(let InterpretedSymbol::Var(#symbol_var_name) = symbols_iter.next().unwrap() else {
                            panic!("Unexpected error");
                        };)
                    }
                }
            }).collect::<Vec<_>>();
            tokens.append_all(quote!(
                // TODO: Don't hardcode types here.
                fn #fun_name(symbols: Vec<InterpretedSymbol<char, i32>>) -> i32 {
                    let mut symbols_iter = symbols.into_iter();
                    #(
                        #bindings
                    )*
                    #action
                }
                rules.push(#fun_name);
            ))
        }
    }
}
