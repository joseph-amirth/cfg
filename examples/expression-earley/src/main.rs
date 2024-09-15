use std::io::stdin;

use cfg::{
    grammar,
    parsing::{EarleyParser, Parser},
};

fn main() {
    let (cfg, vars) = grammar!(
        term => 'A' | 'B' | 'C' | 'D'
        expression1 => expression2 | expression2 '+' expression1 | expression2 '-' expression1
        expression2 => expression3 | expression3 '*' expression2 | expression3 '/' expression2
        expression3 => term | '(' expression1 ')'
    );

    let earley_parser = EarleyParser::of(vars.expression1, cfg);

    loop {
        let input = {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            buffer.retain(|c| !c.is_whitespace());
            buffer
        };

        let word: Vec<char> = input.chars().collect();
        if earley_parser.test(word) {
            println!("Your word is an expression!");
        } else {
            println!("Your word isn't an expression");
        }
    }
}
