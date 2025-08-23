use std::collections::HashMap;
pub struct ReorderedPower;
impl ReorderedPower {
    pub fn reordered_power_of_two(n: i32) -> bool {
        let mut powers: HashMap<i32, i32> = HashMap::new();
        for i in 0..30 {
            powers.insert(i, 2i32.pow(i as u32));
        }
        todo!("Implement reordered power")
    }
}