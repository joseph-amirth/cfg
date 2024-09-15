mod cyk;
mod earley;

pub use cyk::*;
pub use earley::*;

use crate::{Term, Var};

pub trait Parser<W> {
    type TermType: Term;

    fn test(&self, word: W) -> bool {
        self.parse(word).is_some()
    }

    fn parse(&self, word: W) -> Option<ParseTree<Self::TermType>>;
}

#[derive(Debug, Clone)]
pub struct ParseTree<T: Term> {
    root: Var,
    children: Vec<ParsedSymbol<T>>,
}

#[derive(Debug, Clone)]
pub enum ParsedSymbol<T: Term> {
    Term(T),
    Var(Box<ParseTree<T>>),
}
