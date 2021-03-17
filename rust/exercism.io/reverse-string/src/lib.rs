use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // first solution
    // input.chars().rev().collect()

    // second solution, still does not capture grapheme edge case.
    // let mut vec = input
    //     .chars()
    //     .map(|c| c.to_string())
    //     .collect::<Vec<String>>();
    // vec.reverse();
    // vec.join("")

    // 2.5th solution (not sure what i expected)
    // input.rsplit("").collect()

    // final solution checking for the community for answers. The input string is a grapheme
    // cluster so in order to parse it correctly we use the external crate unicode_segmentation.
    input.graphemes(true).rev().collect()
}
