//This is my solution to Leetcode 1768 Merge Strings Alternatively. Here is the problem statement. 
//You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string. Return the merged string.

//solution
//Note: my solution does not handle general strings in rust, because Rust strings can take UTF-8 characters which may occupy more than one byte, here .get() may slice things in the wrong place for non-ASCII  characters


fn merge_alternately(word1: String, word2: String) -> String {
        let mut merge = String::new(); 
        let length1 = word1.len();
        let length2 = word2.len();
        let length = std::cmp::max(length1, length2);
        for i in 0..=length {
            let slice1 = word1.get(i..i+1).unwrap_or("");
            let slice2 = word2.get(i..i+1).unwrap_or("");
            merge.push_str(slice1);
            merge.push_str(slice2);
        }
         merge
    }

fn main() {
    let word1 = String::from("abc");
    let word2 = String::from("pqrst");
    let merged = merge_alternately(word1, word2);
    println!("{}", merged); // "apbqcr"
}