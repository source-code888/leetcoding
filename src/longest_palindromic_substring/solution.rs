/***
    Given a string s, return the longest palindromic substring in s.
    Example 1:
    Input: s = "babad"
    Output: "bab"
    Explanation: "aba" is also a valid answer.
    Example 2:
    Input: s = "cbbd"
    Output: "bb"
*/

pub struct Solution;

impl Solution {
    pub fn longest_palindromic(s: String) -> String {
        let vec: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        let len = s.len();
        for c in 0..len{
            let mut left = c;
            let mut right = c;
            while right + 1 < len && vec[right + 1] == vec[left] {
                right += 1;
            }
            while right + 1 < len && left > 0 && vec[right + 1] == vec[left - 1] {
                right += 1;
                left -= 1;
            }
            if right - left > end - start {
                end = right;
                start = left;
            }
        }
        vec[start..=end].iter().collect()
    }
}