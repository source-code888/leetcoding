pub struct ReverseInteger;

impl ReverseInteger {
    pub fn reverse(x: i32) -> i64 {
        let mut x = x;
        let mut result: i64 = 0;
        while x != 0{
            let next = x % 10;
            x /= 10;
            result = result * 10 + next as i64;
        }
        result
    }
}