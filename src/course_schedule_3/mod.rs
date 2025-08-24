use std::collections::BinaryHeap;
/// # Course Schedule III
/// This problem is listed on leetcode problems set
/// URL: https://leetcode.com/problems/course-schedule-iii/description/
/// # Description
/// There are n different online courses numbered from 1 to n.
/// You are given an array courses where courses[i] = [durationi, lastDayi]
/// indicate that the ith course should be taken continuously for durationi days
/// and must be finished before or on lastDayi.
/// You will start on the 1st day and you cannot take two or more courses simultaneously.
/// Return the maximum number of courses that you can take.
/// Example 1:
/// Input: courses = [[100,200],[200,1300],[1000,1250],[2000,3200]]
/// Output: 3
/// Explanation:
/// There are totally 4 courses, but you can take 3 courses at most:
/// First, take the 1st course, it costs 100 days so you will finish it on the 100th day,
/// and ready to take the next course on the 101st day.
/// Second, take the 3rd course, it costs 1000 days so you will finish it on the 1100th day,
/// and ready to take the next course on the 1101st day.
/// Third, take the 2nd course, it costs 200 days so you will finish it on the 1300th day.
/// The 4th course cannot be taken now, since you will finish it on the 3300th day, which exceeds the closed date.
pub struct Solution;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        if courses.len() < 1 || courses.len() as i32 > 10i32.pow(4u32) {
            return 0i32;
        }
        let mut courses = courses;
        let mut total_time: i32 = 0i32;
        let mut total_courses: i32 = 0i32;
        let mut heap: BinaryHeap<i32> = BinaryHeap::new(); // Priority queue
        courses.sort_by_key(|course| course[1]); // Sort courses based on their deadline
        for course in courses {
            total_courses += 1;
            total_time += course[0];
            heap.push(course[0]);
            if total_time > course[1] {
                let time = heap.pop().unwrap();
                total_time -= time;
                total_courses -= 1;
            }
        }
        total_courses
    }
}
