use std::{collections::HashMap, f64::NAN, usize};
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();
    for n in nums {
        map.entry(n).and_modify(|freq| *freq += 1).or_insert(1);
    }
    let mut count: i32 = 0;
    while count < k {
        if let Some((&max_key, &max_value)) = map.iter().max_by_value(|&(_, v)| v) {
        res.push(*map.values().max().unwrap());
        map.remove(map.values().max().unwrap());
    }
    res
}
fn rust(a:u32)->bool{
    
}
fn main() {
    let nums = vec![3, 3, 2, 1, 1, 1, 2];
    top_k_frequent(nums, 2);
    println!("Hello, world!");
}