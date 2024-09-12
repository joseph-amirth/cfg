use crate::{Grammar, RuleSet, Symbol};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for Grammar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut vars = Vec::new();
        for rule_set in &self.rule_sets {
            vars.push(&rule_set.head);
        }

        vars.sort();
        vars.dedup();

        let rule_sets = &self.rule_sets;

        tokens.append_all(quote!({
            let mut cfg = cfg::Cfg::new();
            #( let #vars = cfg.add_var(); )*
            struct Vars {
                #( #vars: cfg::Var ),*
            }
            #( #rule_sets )*
            (
                cfg,
                Vars {
                    #( #vars ),*
                }
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
                cfg.add_rule(
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
