use crate::{
    parse::{ParseTree, ParsedSymbol},
    Term,
};

pub struct Interpreter<T: Term, M> {
    rules: Vec<fn(Vec<InterpretedSymbol<T, M>>) -> M>,
}

impl<T: Term, M> Interpreter<T, M> {
    pub fn new(rules: Vec<fn(Vec<InterpretedSymbol<T, M>>) -> M>) -> Self {
        Self { rules }
    }

    pub fn interpret(&self, parse_tree: ParseTree<T>) -> M {
        let interpreted_symbols = parse_tree
            .children
            .into_iter()
            .map(|child| match child {
                ParsedSymbol::Term(term) => InterpretedSymbol::Term(term),
                ParsedSymbol::Var(parse_tree) => {
                    InterpretedSymbol::Var(self.interpret(*parse_tree))
                }
            })
            .collect();
        (self.rules[parse_tree.rule_idx])(interpreted_symbols)
    }
}

pub enum InterpretedSymbol<T: Term, M> {
    Term(T),
    Var(M),
}
