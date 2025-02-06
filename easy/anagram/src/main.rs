use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut freq: [u32; 26] = [0; 26];
        if s.len() != t.len() {
            return false;
        }
        while i < s.len() {
            if let Some(c) = s.chars().nth(i) {
                freq[((c) as u8 - 97) as usize] += 1;
            } else {
                return false;
            }
            i += 1;
        }
        while j < t.len() {
            if let Some(c) = t.chars().nth(j) {
                if freq[((c) as u8 - 97) as usize] == 0 {
                    return false;
                }
                freq[((c) as u8 - 97) as usize] -= 1;
            } else {
                return false;
            }
            j += 1;
        }
        return true;
    }
}
fn main() {}
