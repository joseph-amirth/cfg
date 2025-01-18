# cfg (name TBD)

Rust crate to

- construct and manipulate context-free grammars over generic symbol sets,
- test for membership and parse words in the corresponding languages,
- compute semantics of words belonging to a grammar by supplying semantic rules.

## Features

A brief demonstration of each feature is given below. For more detailed and executable examples, see
[cfg/examples](cfg/examples).

### Constructing Grammars

One can use the `grammar!` macro to easily construct a grammar.

```rust
let cfg = grammar!(
    expr => sum
    sum => product | product '+' sum | product '-' sum
    product => term | term '*' product | term '/' product
    term => unit | '(' expr ')'
    unit => 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
);
```

### Membership Testing

One can create a parser implementing a supported algorithm and use the `test` method on the parser
on a word of a supported type to determine membership.

```rust
let parser = CykParser::of(cfg);

let expression = "A+B*(C-D/E)+F*(G)";
let expression: Vec<char> = expression.chars().collect();
assert!(parser.test(expression));
```

### Constructing Parse Trees

One can alternatively ask the parser to return a parse tree if possible using the `parse` method.
This parse tree can be formatted as shown below.

```rust
earley_parser = EarleyParser::of(cfg);

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
```

See [`parse-tree.txt`](parse-tree.txt) for the output or run
[cfg/examples/format-parse-tree.rs](cfg/examples/format-parse-tree.rs).

### Compute Semantics

One can supply semantic rules along with each syntax rule of the grammar by using the
`interpreted_grammar!` macro, and thus construct a grammar along with an interpreter for it. The
interpreter's `interpret` method may be used to compute the semantics of a parsed word.

```rust
let (cfg, interpreter) = interpreted_grammar!(
    char, f64,
    expr => sum { _1 }
    sum => product { _1 } | product '+' sum { _1 + _3 } | product '-' sum { _1 - _3 }
    product => term { _1 } | term '*' product { _1 * _3 } | term '/' product { _1 / _3 }
    term => number { _1 } | '(' expr ')' { _2 }
    number => digit { _1 } | number digit { _1 * (10 as f64) + _2 }
    digit =>
        '0' { 0.0 } |
        '1' { 1.0 } |
        '2' { 2.0 } |
        '3' { 3.0 } |
        '4' { 4.0 } |
        '5' { 5.0 } |
        '6' { 6.0 } |
        '7' { 7.0 } |
        '8' { 8.0 } |
        '9' { 9.0 }
);

let parser = EarleyParser::of(cfg);

let expression = "(10*3)/2+7";
let expression: Vec<char> = expression.chars().collect();
let Some(parse_tree) = parser.parse(expression) else {
    println!("Ill-formed expression");
    continue;
};

// Prints out 22.
println!("{}", interpreter.interpret(parse_tree));
```
