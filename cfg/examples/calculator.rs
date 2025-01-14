use cfg::interpreted_grammar;

fn main() {
    let cfg = interpreted_grammar!(
        expr => sum { _1 }
        sum => product { _1 } | product '+' sum { _1 + _3 } | product '-' sum { _1 - _3 }
        product => term { _1 } | term '*' product { _1 * _3 } | term '/' product { _1 / _3 }
        term => number { _1 } | '(' expr ')' { _2 }
        number => digit { _1 } | number digit { _1 * 10 + _2 }
        digit =>
            '1' { 1 } |
            '2' { 2 } |
            '3' { 3 } |
            '4' { 4 } |
            '5' { 5 } |
            '6' { 6 } |
            '7' { 7 } |
            '8' { 8 } |
            '9' { 9 }
    );
}
