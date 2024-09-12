mod cyk;
mod earley;

pub use cyk::*;
pub use earley::*;

pub trait Parser<W> {
    type ParseTree;

    fn test(&self, word: W) -> bool {
        self.parse(word).is_some()
    }

    fn parse(&self, word: W) -> Option<Self::ParseTree>;
}
