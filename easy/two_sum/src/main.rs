use std::{collections::HashMap, usize};
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut hash: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            hash.insert(*n, i);
        }
        for (i, n) in nums.iter().enumerate() {
            match hash.get(&(target - *n)) {
                Some(&val) => {
                    if val == i {
                        continue;
                    }
                    res.push(val as i32);
                    res.push(i as i32);
                    return res;
                }
                _ => {
                    continue;
                }
            }
        }
        res
    }
}
fn main() {
    let nums = vec![3, 2, 4];
    println!("{:?}", two_sum(nums, 6));
}
