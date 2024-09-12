use cfg::{grammar, parsing::CykParser, parsing::Parser};

#[test]
fn test_cnf() {
    let (dyck_grammar, vars) = grammar!(
        start => '(' ')'
        start => '(' start ')'
        start => start start
    );

    let cyk_parser = CykParser::of(dyck_grammar, vars.start);

    for len in 1..10 {
        for (brkt_seq, expectation) in brkt_seq_test_cases(len) {
            for ch in &brkt_seq {
                print!("{}", ch);
            }
            print!("\n");
            assert_eq!(cyk_parser.test(brkt_seq), expectation);
        }
    }
}

fn brkt_seq_test_cases(len: usize) -> Vec<(Vec<char>, bool)> {
    let mut brkt_seqs = Vec::new();
    for mask in 0..1 << len {
        let mut brkt_seq = Vec::new();
        for i in 0..len {
            if ((mask >> i) & 1) == 0 {
                brkt_seq.push('(');
            } else {
                brkt_seq.push(')');
            }
        }
        let expectation = is_balanced(&brkt_seq);
        brkt_seqs.push((brkt_seq, expectation));
    }
    brkt_seqs
}

fn is_balanced(brkt_seq: &Vec<char>) -> bool {
    let mut balance = 0;

    let mut is_balanced = true;
    for ch in brkt_seq {
        if *ch == '(' {
            balance += 1;
        } else {
            balance -= 1;
        }
        is_balanced &= balance >= 0;
    }

    is_balanced &= balance == 0;
    is_balanced
}
