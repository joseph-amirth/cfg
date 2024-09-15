mod cyk;
mod earley;

use std::{collections::HashMap, fmt::Display};

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

pub struct ParseTreeFormatter<'a, T: Term> {
    parse_tree: &'a ParseTree<T>,
    options: FormatOptions<'a>,
}

pub struct FormatOptions<'a> {
    pub vars_map: &'a HashMap<Var, &'a str>,
    pub indendation: usize,
    pub style: FormatStyle,
}

pub struct FormatStyle {
    horizontal: char,
    vertical: char,
    vertical_and_right: char,
    up_and_right: char,
}

pub const ASCII: FormatStyle = FormatStyle {
    horizontal: '-',
    vertical: '|',
    vertical_and_right: '|',
    up_and_right: '`',
};

pub const BOX_DRAWING: FormatStyle = FormatStyle {
    horizontal: '═',
    vertical: '║',
    vertical_and_right: '╠',
    up_and_right: '╚',
};

impl<'a, T: Term + Display> Display for ParseTreeFormatter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bars = Vec::<bool>::new();
        self.parse_tree.fmt_impl(&self.options, &mut bars, f)?;
        Ok(())
    }
}

impl<T: Term + Display> ParseTree<T> {
    pub fn fmt_with_options<'a>(&'a self, options: FormatOptions<'a>) -> ParseTreeFormatter<'a, T> {
        ParseTreeFormatter {
            parse_tree: &self,
            options,
        }
    }

    fn fmt_impl(
        &self,
        options: &FormatOptions,
        bars: &mut Vec<bool>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        fmt_parse_tree_node(&options.vars_map[&self.root], options, bars, f)?;
        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i + 1 == self.children.len();
            bars.push(!is_last_child);
            match child {
                ParsedSymbol::Term(term) => {
                    fmt_parse_tree_node(term, options, bars, f)?;
                }
                ParsedSymbol::Var(parse_tree) => {
                    parse_tree.fmt_impl(options, bars, f)?;
                }
            }
            bars.pop();
        }
        Ok(())
    }
}

fn fmt_parse_tree_node(
    node: &impl Display,
    options: &FormatOptions,
    bars: &Vec<bool>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    if bars.len() > 1 {
        for i in 0..bars.len() - 1 {
            if bars[i] {
                write!(f, "{}", options.style.vertical)?;
            } else {
                write!(f, " ")?;
            }
            for _ in 0..options.indendation + 1 {
                write!(f, " ")?;
            }
        }
    }
    if !bars.is_empty() {
        if *bars.last().unwrap() {
            write!(f, "{}", options.style.vertical_and_right)?;
        } else {
            write!(f, "{}", options.style.up_and_right)?;
        }
        for _ in 0..options.indendation {
            write!(f, "{}", options.style.horizontal)?;
        }
        write!(f, "{}", " ");
    }
    write!(f, "{}\n", node)?;
    Ok(())
}
