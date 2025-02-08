fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![1; nums.len()];

    let mut mul: i32 = nums[nums.len() - 1];
    for i in (0..nums.len() - 1).rev() {
        res[i] = mul;
        mul *= nums[i];
    }
    mul = nums[0];
    for i in 1..nums.len() {
        res[i] *= mul;
        mul *= nums[i];
    }
    res
}
fn main() {
    let vec = vec![1, 2, 3, 4];
    product_except_self(vec);
}
