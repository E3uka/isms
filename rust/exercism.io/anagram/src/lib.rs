use std::collections::HashSet;

pub fn anagrams_for_2<'a, 'b>(word: &'a str, possible_anagrams: &[&'b str]) -> HashSet<&'b str> {
    possible_anagrams
        .iter()
        .filter(|&test_word| {
            // lengths are different so they cannot be the same.
            if word.len() != test_word.len() {
                return false;
            }

            let mut word: Vec<char> = word.to_lowercase().chars().collect();
            let mut test_word: Vec<char> = test_word.to_lowercase().chars().collect();

            // a word is always an anagram of itself, we omit the same word in the answer.
            if word != test_word {
                word.sort_unstable();
                test_word.sort_unstable();
                return word.eq(&test_word);
            }

            false
        })
        .cloned()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}

// suggested community mentor solution, slightly better performance compared to above:
// 3,001 ns/iter (+/- 88) vs 2,495 ns/iter (+/- 87)
//
// Clone is explicit and might be expensive and Copy is implicit and cannot be re-implemented
// in the below case because we are returning reference to values that already exist and must live
// to satisfy the lifetime of the return variable, it makes sense to do a .copied() operation
// instead of a .clone() on line 52. This could explain the diffrence in performance in community
// solution.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == word_lower.len() // lengths must be equal.
                && candidate_lower != word_lower // words are not initially the same.
                && get_sorted(&candidate_lower) == word_sorted // anagram contains all of word.
        })
        .copied()
        .collect()
}
