//! # Letter Combinations of a Phone Number
//! ## Problem Description:
//! Given a string containing digits from 2-9 inclusive,
//! return all possible letter combinations that the number could represent.
//! Return the answer in any order.
//! A mapping of digits to letters (just like on the telephone buttons) is given below.
//! Note that 1 does not map to any letters.
//!
//! ## Approach
//! It can be solved with recursion but it's very common to see that solution, so I rather did what is in the code. It sucks btw.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut map: HashMap<char, Vec<&'static str>> = HashMap::new();
        map.insert('2', vec!["a", "b", "c"]);
        map.insert('3', vec!["d", "e", "f"]);
        map.insert('4', vec!["g", "h", "i"]);
        map.insert('5', vec!["j", "k", "l"]);
        map.insert('6', vec!["m", "n", "o"]);
        map.insert('7', vec!["p", "q", "r", "s"]);
        map.insert('8', vec!["t", "u", "v"]);
        map.insert('9', vec!["w", "x", "y", "z"]);
        let mut combinations: Vec<String> = vec![];
        let digit = digits.chars().nth(digits.len() - 1).unwrap();
        let letters = map.get(&digit).unwrap().clone();
        for l in letters {
            combinations.push(l.to_string());
        }
        let digits = &digits[0..digits.len() - 1];
        for d in digits.chars().rev() {
            let letters = map.get(&d).unwrap().clone();
            let suffix = combinations.clone();
            combinations.clear();
            for l in letters {
                for s in suffix.iter() {
                    combinations.push(format!("{l}{s}"))
                }
            }
        }
        combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::letter_combinations(String::from("2")),
            ["a", "b", "c"]
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::letter_combinations(String::from("234")),
            [
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
                "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
                "cfg", "cfh", "cfi"
            ]
        )
    }
}
