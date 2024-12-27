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
    let cfg = grammar!(
        expr => sum
        sum => product | product '+' sum | product '-' sum
        product => term | term '*' product | term '/' product
        term => unit | '(' sum ')'
        unit => 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
    );

    let cyk_parser = CykParser::of(cfg);

    let expression = "A+B*(C-D/E)+F*(G)";
    let expression: Vec<char> = expression.chars().collect();
    if cyk_parser.test(expression) {
        println!("It is an expression!");
    } else {
        println!("It isn't an expression");
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
    let cfg = grammar!(
        expr => sum
        sum => product | product '+' sum | product '-' sum
        product => term | term '*' product | term '/' product
        term => unit | '(' sum ')'
        unit => 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
    );

    let earley_parser = EarleyParser::of(cfg);

    let expression: &str = "A+B*(C-D/E)+F*(G)";
    let expression: Vec<char> = expression.chars().collect();

    let parse_tree = earley_parser
        .parse(expression)
        .expect("expression is correctly parsed");

    print!(
        "{}",
        parse_tree.fmt_with_options(FormatOptions {
            indendation: 1,
            style: FormatStyle::BOX_DRAWING,
        })
    );
}
```

The output:

```
expr
╚═ sum
   ╠═ product
   ║  ╚═ term
   ║     ╚═ unit
   ║        ╚═ A
   ╠═ +
   ╚═ sum
      ╠═ product
      ║  ╠═ term
      ║  ║  ╚═ unit
      ║  ║     ╚═ B
      ║  ╠═ *
      ║  ╚═ product
      ║     ╚═ term
      ║        ╠═ (
      ║        ╠═ sum
      ║        ║  ╠═ product
      ║        ║  ║  ╚═ term
      ║        ║  ║     ╚═ unit
      ║        ║  ║        ╚═ C
      ║        ║  ╠═ -
      ║        ║  ╚═ sum
      ║        ║     ╚═ product
      ║        ║        ╠═ term
      ║        ║        ║  ╚═ unit
      ║        ║        ║     ╚═ D
      ║        ║        ╠═ /
      ║        ║        ╚═ product
      ║        ║           ╚═ term
      ║        ║              ╚═ unit
      ║        ║                 ╚═ E
      ║        ╚═ )
      ╠═ +
      ╚═ sum
         ╚═ product
            ╠═ term
            ║  ╚═ unit
            ║     ╚═ F
            ╠═ *
            ╚═ product
               ╚═ term
                  ╠═ (
                  ╠═ sum
                  ║  ╚═ product
                  ║     ╚═ term
                  ║        ╚═ unit
                  ║           ╚═ G
                  ╚═ )
```
