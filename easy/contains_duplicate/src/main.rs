impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut save = HashSet::new();

        for num in nums {
            if !save.insert(num) {
                return true;
            }
        }
        false
    }
}
fn main() {}
