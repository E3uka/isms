enum Bracket {
    Open(char),
    Close(char),
}

pub fn brackets_are_balanced(string: &str) -> bool {
    // if the string is empty it is already balanced.
    if string.is_empty() {
        return true;
    }

    // create vector of closing brackets that correspond to in-order seen variables.
    let mut seen: Vec<char> = Vec::new();
    let mut flag: bool = true;

    for bracket in string {
        match bracket {
                '(' => seen.push(')'),
                '{' => seen.push('}'),
                '[' => seen.push(']'),

                // set flag 
                ')' => flag = ')' == seen.pop().unwrap(),
                '}' => flag = '}' == seen.pop().unwrap(),
                ']' => flag = ']' == seen.pop().unwrap(),

                _ => {/* do nothing */},
        }

        // if flag if false at any stage return early
        if !flag return false;
    }

    flag

    // // if length of resulting vector is odd, brackets are unbalanced; return early.
    // if seen.len() % 2 != 0 {
    //     return false;
    // }

    // // final check that the length of seen is 0.
    // seen.len() == 0
}
