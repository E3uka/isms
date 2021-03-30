enum Bracket {
    Open(char),
    Close(char),
}

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    // reduce input string into vector of only opening or closing brackets.
    let processed_string: Vec<Option<Bracket>> = string
        .chars()
        .map(|c| match c {
            '{' | '(' | '[' => Some(Bracket::Open(c)),
            '}' | ')' | ']' => Some(Bracket::Close(c)),
            _ => None,
        })
        .filter(|x| x.is_some())
        .collect();

    // if length of resulting vector is odd, brackets are already unbalanced
    if processed_string.len() % 2 != 0 {
        return false;
    }

    // create vector of closing brackets that correspond to in-order seen variables.
    let mut seen: Vec<char> = Vec::with_capacity(processed_string.len());

    for elem in processed_string {
        match elem {
            Some(Bracket::Open(c)) => match c {
                '(' => seen.push(')'),
                '{' => seen.push('}'),
                '[' => seen.push(']'),
                _ => unreachable!(), // Other chars are filtered out at the processing step.
            },

            Some(Bracket::Close(c)) => {
                // closing bracket found without opening bracket seen first.
                if seen.len() == 0 {
                    return false;
                }

                // closing bracket does not match opening order.
                if c != seen.pop().unwrap() {
                    return false;
                }
            }

            _ => unreachable!(), // None case is filtered at the processing step.
        }
    }

    // check seen is empty after loop.
    seen.len() == 0
}
