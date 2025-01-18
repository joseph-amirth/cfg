use std::io::{stdin, stdout, Write};

use cfg::{
    interpreted_grammar,
    parse::{EarleyParser, Parser},
};

fn main() {
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

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let input = {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            buffer.retain(|c| !c.is_whitespace());
            buffer
        };

        let word: Vec<char> = input.chars().collect();
        let Some(parse_tree) = parser.parse(word) else {
            println!("Ill-formed expression");
            continue;
        };
        println!("{}", interpreter.interpret(parse_tree));
    }
}
