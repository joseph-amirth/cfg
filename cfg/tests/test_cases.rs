pub fn brkt_seq_test_cases(len: usize) -> Vec<(Vec<char>, bool)> {
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
        let expectation = is_balanced_brkt_seq(&brkt_seq);
        brkt_seqs.push((brkt_seq, expectation));
    }
    brkt_seqs
}

pub fn expression_test_cases() -> &'static [(&'static str, bool)] {
    &[
        ("A", true),
        ("(A)", true),
        ("A+B", true),
        ("A-B", true),
        ("A*B", true),
        ("A/B", true),
        ("A+(B*C)", true),
        ("A+(B*C/D)", true),
        ("+", false),
        ("A+", false),
        ("A+*", false),
        ("A+*B", false),
        ("AB", false),
    ]
}

fn is_balanced_brkt_seq(brkt_seq: &Vec<char>) -> bool {
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
