use cfg::{grammar, parsing::CykParser, parsing::Parser};

mod test_cases;

#[test]
fn cyk_parser_tests_brkt_seqs() {
    let dyck_grammar = grammar!(
        start => '(' ')'
        start => '(' start ')'
        start => start start
    );

    let cyk_parser = CykParser::of(dyck_grammar);

    for len in 1..10 {
        for (brkt_seq, expectation) in test_cases::brkt_seq_test_cases(len) {
            for ch in &brkt_seq {
                print!("{}", ch);
            }
            print!("\n");
            assert_eq!(cyk_parser.test(brkt_seq), expectation);
        }
    }
}

#[test]
fn cyk_parser_tests_expressions() {
    let cfg = grammar!(
        expression1 => expression2 | expression2 '+' expression1 | expression2 '-' expression1
        expression2 => expression3 | expression3 '*' expression2 | expression3 '/' expression2
        expression3 => term | '(' expression1 ')'
        term => 'A' | 'B' | 'C' | 'D'
    );

    let cyk_parser = CykParser::of(cfg);

    for (word, expected_result) in test_cases::expression_test_cases() {
        let word = word.chars().collect();
        assert_eq!(cyk_parser.test(word), *expected_result);
    }
}
