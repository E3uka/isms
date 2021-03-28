use std::collections::HashSet;

pub fn anagrams_for<'a, 'b>(word: &'a str, possible_anagrams: &[&'b str]) -> HashSet<&'b str> {
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
