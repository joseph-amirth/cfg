use cfg::{grammar, parse::EarleyParser, parse::Parser};

mod test_cases;

#[test]
fn earley_parser_tests_brkt_seqs() {
    let dyck_grammar = grammar!(
        start => '(' ')'
        start => '(' start ')'
        start => start start
    );

    let earley_parser = EarleyParser::of(dyck_grammar);

    for len in 1..10 {
        for (brkt_seq, expectation) in test_cases::brkt_seq_test_cases(len) {
            for ch in &brkt_seq {
                print!("{}", ch);
            }
            print!("\n");
            assert_eq!(earley_parser.test(brkt_seq), expectation);
        }
    }
}

#[test]
fn earley_parser_tests_expressions() {
    let cfg = grammar!(
        expression1 => expression2 | expression2 '+' expression1 | expression2 '-' expression1
        expression2 => expression3 | expression3 '*' expression2 | expression3 '/' expression2
        expression3 => term | '(' expression1 ')'
        term => 'A' | 'B' | 'C' | 'D'
    );

    let earley_parser = EarleyParser::of(cfg);

    for (word, expected_result) in test_cases::expression_test_cases() {
        let word = word.chars().collect();
        assert_eq!(earley_parser.test(word), *expected_result);
    }
}
