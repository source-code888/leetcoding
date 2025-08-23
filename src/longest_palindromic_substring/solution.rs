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
        let len = s.len();
        if len == 0 {
            return s;
        }
        let vec: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = 0;
        let mut start = 0;
        let mut end = 0;
        while right < len{
            if right + 1 < len && vec[left] == vec[right + 1] {
                right += 1
            }
            if right + 1 < len && left > 0 && vec[left - 1] == vec[right + 1] {
                right += 1;
                left -= 1
            }
            if right - left > end - start{
                end = right;
                start = left
            }
        }
        s[start.. end].to_string()
    }
    pub fn longest_palindromic2(s: String) -> String {
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