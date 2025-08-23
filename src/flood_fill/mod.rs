use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        if image.is_empty() {
            return vec![];
        }
        let mut image: Vec<Vec<i32>> = image;
        let m: usize = image.len();
        let n = image[0].len();
        if m <= 0 || n > 50 || color >= 2i32.pow(16u32) || sc < 0 || sc >= m as i32 || sr < 0 || sc >= n as i32 {
            return vec![];
        }
        // Create a visited nodes set
        let mut visited_nodes: Vec<(i32, i32)> = vec![];
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let start_color: i32 = image[sr as usize][sc as usize];
        queue.push_back((sr, sc));
        while queue.len() > 0 {
            let (sr, sc) = queue.pop_front().unwrap();
            visited_nodes.push((sr, sc));
            image[sr as usize][sc as usize] = color;
            for neighbor in Solution::neighbors(image.clone(), sr, sc, start_color) {
                if !visited_nodes.contains(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
        image
    }

    fn neighbors(img: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<(i32, i32)> {
        let neighbors_positions: [(i32, i32); 4] = [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ];
        let mut neighbors: Vec<(i32, i32)> = vec![];
        for neighbor in neighbors_positions {
            if neighbor.0 >= 0
                && neighbor.0 < img.len() as i32
                && neighbor.1 >= 0
                && neighbor.1 < img[0].len() as i32
                && img[neighbor.0 as usize][neighbor.1 as usize] == color
            {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}
