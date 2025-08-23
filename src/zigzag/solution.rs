pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 || num_rows >= s.len() as i32 {
            return s;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut result:String = String::new();
        let diag_size: i32 = num_rows - 2;
        let step: i32 = num_rows + diag_size;
        for i in 0..num_rows{
            let k: i32 = if i == 0 || i == num_rows - 1 { 0 } else { i * 2 };
            result.push(chars[i as usize]);
            let mut j: i32 = i + step;
            while j < s.len() as i32{
                if k != 0 && j - k != i && i < num_rows - 1{
                    result.push(chars[(j - k) as usize])
                }
                result.push(chars[j as usize]);
                j += step
            }
            if i < num_rows - 1 && j >= s.len() as i32 && j - k < s.len() as i32{
                result.push(chars[(j - k) as usize])
            }
        }
        result
    }

    pub fn convert_to_numbers(s: String, num_rows: i32) -> Vec<i32> {
        if num_rows <= 1 || num_rows >= s.len() as i32 {
            return (0..s.len() as i32).collect::<Vec<i32>>();
        }
        let mut result:Vec<i32> = vec![];
        let diag_size: i32 = num_rows - 2;
        let step: i32 = num_rows + diag_size;
        for i in 0..num_rows{
            let k: i32 = if i == 0 || i == num_rows - 1 { 0 } else {i * 2};
            result.push(i);
            let mut j = i + step;
            while j < s.len() as i32{
                if k != 0 && j - k != i{
                    result.push(j - k)
                }
                result.push(j);
                j += step
            }
            if i < num_rows - 1 && j >= s.len() as i32 && j - k < s.len() as i32{
                result.push(j - k)
            }
        }
        result
    }
}