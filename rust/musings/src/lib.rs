// for testing the parity of numbers within a set, you can use the XOR operator with the
// knowledge of its properties:
//
// XOR of a number with 0 is itself.
// XOR of a number with itself is 0.
// XOR is commutative so the order of the operator does not matter.
//
// with this knowledge we can find out what number in a list of matching numbers does not have
// a pair.
fn check_list_parity(list: &[isize]) -> isize {
    // let mut zero = 0;
    // list.iter().for_each(|num| zero ^= num);
    // zero

    // more concise
    list.iter().fold(0, |a, b| a ^ b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parity() {
        let positive_list = vec![1, 1, 2, 2, 3, 4, 4, 5, 5];
        let negative_list = vec![-1, -1, -2, -2, -3, -3, -5];
        assert_eq!(check_list_parity(&positive_list), 3);
        assert_eq!(check_list_parity(&negative_list), -5);
    }
}
