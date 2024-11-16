# cfg (name TBD)

A library to construct and manipulate context-free grammars over generic symbol sets and test for
membership of and parse words in the corresponding context-free languages.

## Examples

1. Test whether a word can be derived from a given CFG
   ([examples/expression-cyk](examples/expression-cyk)).

```rust
use cfg::{
    grammar,
    parsing::{CykParser, Parser},
};

fn main() {
    let (cfg, vars, _) = grammar!(
        term => 'A' | 'B' | 'C' | 'D'
        expression1 => expression2 | expression2 '+' expression1 | expression2 '-' expression1
        expression2 => expression3 | expression3 '*' expression2 | expression3 '/' expression2
        expression3 => term | '(' expression1 ')'
    );

    let cyk_parser = CykParser::of(cfg, vars.expression1);

    let word: Vec<char> = ...;
    if cyk_parser.test(word) {
        println!("Your word is an expression!");
    } else {
        println!("Your word isn't an expression");
    }
}
```

2. Print out a derivation (parse tree) of a word belonging to the language of a given CFG
   ([examples/parse-tree-formatting](examples/parse-tree-formatting)).

```rust
use cfg::{
    grammar,
    parsing::{EarleyParser, FormatOptions, FormatStyle, Parser},
};

fn main() {
    let (cfg, vars, map) = grammar!(
        term => 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
        expression1 => expression2 | expression2 '+' expression1 | expression2 '-' expression1
        expression2 => expression3 | expression3 '*' expression2 | expression3 '/' expression2
        expression3 => term | '(' expression1 ')'
    );

    let vars_map = &map.into_iter().map(|(k, v)| (v, k)).collect();

    let earley_parser = EarleyParser::of(vars.expression1, cfg);

    let expression: &str = "A+B*(C-D/E)+F*(G)";
    let expression: Vec<char> = expression.chars().collect();

    let parse_tree = earley_parser
        .parse(expression)
        .expect("expression is correctly parsed");

    print!(
        "{}",
        parse_tree.fmt_with_options(FormatOptions {
            vars_map,
            indendation: 1,
            style: FormatStyle::BOX_DRAWING,
        })
    );
}
```

The output:

```
expression1
╠═ expression2
║  ╚═ expression3
║     ╚═ term
║        ╚═ A
╠═ +
╚═ expression1
   ╠═ expression2
   ║  ╠═ expression3
   ║  ║  ╚═ term
   ║  ║     ╚═ B
   ║  ╠═ *
   ║  ╚═ expression2
   ║     ╚═ expression3
   ║        ╠═ (
   ║        ╠═ expression1
   ║        ║  ╠═ expression2
   ║        ║  ║  ╚═ expression3
   ║        ║  ║     ╚═ term
   ║        ║  ║        ╚═ C
   ║        ║  ╠═ -
   ║        ║  ╚═ expression1
   ║        ║     ╚═ expression2
   ║        ║        ╠═ expression3
   ║        ║        ║  ╚═ term
   ║        ║        ║     ╚═ D
   ║        ║        ╠═ /
   ║        ║        ╚═ expression2
   ║        ║           ╚═ expression3
   ║        ║              ╚═ term
   ║        ║                 ╚═ E
   ║        ╚═ )
   ╠═ +
   ╚═ expression1
      ╚═ expression2
         ╠═ expression3
         ║  ╚═ term
         ║     ╚═ F
         ╠═ *
         ╚═ expression2
            ╚═ expression3
               ╠═ (
               ╠═ expression1
               ║  ╚═ expression2
               ║     ╚═ expression3
               ║        ╚═ term
               ║           ╚═ G
               ╚═ )
```
