use cfg::{
    grammar,
    parsing::{EarleyParser, FormatOptions, FormatStyle, Parser},
};

fn main() {
    let (cfg, map) = grammar!(
        expression1 => expression2 | expression2 '+' expression1 | expression2 '-' expression1
        expression2 => expression3 | expression3 '*' expression2 | expression3 '/' expression2
        expression3 => term | '(' expression1 ')'
        term => 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
    );

    let vars_map = &map.into_iter().map(|(k, v)| (v, k)).collect();

    let earley_parser = EarleyParser::of(cfg);

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
