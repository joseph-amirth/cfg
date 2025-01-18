use crate::{Grammar, InterpretedGrammar, InterpretedRuleSet, RuleSet, Symbol};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::iter::Iterator;
use syn::{LitStr, Type};

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
        let term_type = &self.term_type;
        let meaning_type = &self.meaning_type;
        let interpreted_rule_sets = self
            .interpreted_rule_sets
            .iter()
            .map(|interpreted_rule_set| interpreted_rule_set.to_tokens(term_type, meaning_type))
            .collect::<Vec<_>>();
        tokens.append_all(quote!({
            use cfg::interpret::InterpretedSymbol;
            let mut rules = Vec::<fn(Vec<InterpretedSymbol<#term_type, #meaning_type>>) -> #meaning_type>::new();
            #( #interpreted_rule_sets )*
            (#grammar, cfg::interpret::Interpreter::new(rules))
        }));
    }
}

impl InterpretedRuleSet {
    fn to_tokens(&self, term_type: &Type, meaning_type: &Type) -> TokenStream {
        let mut tokens = TokenStream::new();

        let head = &self.head;
        self.bodies.iter().enumerate().map(|(i, (symbols, action))| {
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
            quote!(
                #[allow(unused_braces)]
                fn #fun_name(symbols: Vec<InterpretedSymbol<#term_type, #meaning_type>>) -> #meaning_type {
                    let mut symbols_iter = symbols.into_iter();
                    #(
                        #bindings
                    )*
                    #action
                }
                rules.push(#fun_name);
            )
        }).for_each(|token_stream| tokens.append_all(token_stream));

        tokens
    }
}
