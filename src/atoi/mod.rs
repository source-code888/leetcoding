
pub struct MyAtoi;

impl MyAtoi {
    pub fn solve(s: String) -> i32{
        let s = s.trim();
        let chars: Vec<char> = s.chars().collect();
        let mut p: String = String::new();
        for i in 0..chars.len() {
            if chars[i].is_numeric(){
                p.push(chars[i])
            }else if i == 0 && (chars[i] == '-' || chars[i] == '+') {
                p.push(chars[i])
            }else{
                break
            }
        }
        p.parse::<i32>().unwrap_or_else(
            |e| {
                match e.kind(){
                    std::num::IntErrorKind::PosOverflow => {i32::MAX}
                    std::num::IntErrorKind::NegOverflow => {i32::MIN}
                    _ => {0}
                }
            }
        )
    }
}