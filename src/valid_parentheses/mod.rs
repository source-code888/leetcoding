use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st: Vec<char> = vec![];
        for c in s.chars().collect::<Vec<char>>() {
            match c {
                '(' | '[' | '{' => st.push(c),
                _ => {
                    if st
                        .pop_if(|t| {
                            *t == match c {
                                ')' => '(',
                                ']' => '[',
                                _ => '{',
                            }
                        })
                        .is_none()
                    {
                        return false;
                    }
                }
            }
        }
        st.is_empty()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_1() {
        assert!(Solution::is_valid(String::from("()")))
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_valid(String::from("(")))
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_valid(String::from("()[]{}")))
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_valid(String::from("(]")))
    }

    #[test]
    fn test_5() {
        assert!(Solution::is_valid(String::from("[(){}]({[]})")))
    }

    #[test]
    fn test_6() {
        assert!(!Solution::is_valid(String::from("[(){}]({[]}))")))
    }

    #[test]
    fn test_7() {
        assert!(!Solution::is_valid(String::from(")")))
    }

    #[test]
    fn test_8() {
        assert!(!Solution::is_valid(String::from("([)]")))
    }
}
