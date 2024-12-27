use crate::{Grammar, GrammarWithActions, RuleSet, RuleSetWithActions, Symbol};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::LitStr;

impl ToTokens for Grammar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut vars = Vec::new();
        for rule_set in &self.rule_sets {
            vars.push(&rule_set.head);
        }

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
            (
                cfg_builder.build(#start_var),
                std::collections::HashMap::<&str, cfg::Var>::from([
                    #(( #var_lits, #vars )),*
                ])
            )
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

impl ToTokens for GrammarWithActions {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!();
    }
}

impl ToTokens for RuleSetWithActions {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!();
    }
}
