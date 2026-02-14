use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into();
        let chars = s.chars().collect::<Vec<char>>();
        let mut ans: i32 = 0;
        let mut i: usize = 0;
        while i < chars.len() {
            let curr_v: i32 = map[&chars[i]];
            if i + 1 < chars.len() && map[&chars[i + 1]] > curr_v {
                ans += map[&chars[i + 1]] - curr_v;
                i += 2
            } else {
                ans += curr_v;
                i += 1
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994)
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::roman_to_int(String::from("XXIX")), 29)
    }
}
