use std::io::{stdin, stdout, Write};

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

    loop {
        print!("Enter an expression: ");
        stdout().flush().unwrap();
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
        println!();
    }
}
