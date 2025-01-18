use cfg::prelude::*;

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
