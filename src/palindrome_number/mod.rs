/// # Palindrome Number
/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/palindrome-number/description/
/// # Description:
/// Given an integer x, return true if x is a palindrome, and false otherwise.
pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut x = x;
        let y = x;
        let mut r:i32 = 0;
        while x != 0 {
            let next = x % 10i32;
            x /= 10;
            let (a, b) = r.overflowing_mul(10);
            if b { break; }
            r = a + next
        }
        r == y
    }
}



#[cfg(test)]
mod tests {
    use crate::palindrome_number::Solution;


    #[test]
    fn test(){
        let (x , y ) = i32::MAX.overflowing_mul(100);
        println!("{}, {y}", x);
        assert!(y);
    }
    /// CASE 1: 202 is palindrome
    #[test]
    fn case_1() {
        let x = 202;
        let is_palindrome = Solution::is_palindrome(x);
        println!("x: {x}\nis_palindrome: {is_palindrome}");
        assert!(is_palindrome);
    }

    #[test]
    fn case_2() {
        let x = 121;
        let is_palindrome = Solution::is_palindrome(x);
        println!("x: {x}\nis_palindrome: {is_palindrome}");
        assert!(is_palindrome);
    }

    #[test]
    fn case_3() {
        let x = 232;
        let is_palindrome = Solution::is_palindrome(x);
        println!("x: {x}\nis_palindrome: {is_palindrome}");
        assert!(is_palindrome);
    }

    #[test]
    fn case_4() {
        let x = 435;
        let is_palindrome = Solution::is_palindrome(x);
        println!("x: {x}\nis_palindrome: {is_palindrome}");
        assert_ne!(is_palindrome, true); // must be false since 435 is not palindrome
    }

    #[test]
    fn case_5() {
        let x = 1000000001;
        let is_palindrome = Solution::is_palindrome(x);
        println!("x: {x}\nis_palindrome: {is_palindrome}");
        assert!(is_palindrome); // must be true
    }

    #[test]
    fn case_6() {
        let x = 2i32.pow(10);
        let is_palindrome = Solution::is_palindrome(x);
        println!("x: {x}\nis_palindrome: {is_palindrome}");
        assert_ne!(is_palindrome, true); // must be false
    }
}