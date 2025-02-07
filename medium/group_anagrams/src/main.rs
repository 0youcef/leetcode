use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut freq: Vec<[i32; 26]> = vec![[0; 26]; strs.len()];

    for (i, s) in strs.iter().enumerate() {
        for c in s.chars() {
            if let Some(freq_arr) = freq.get_mut(i) {
                freq_arr[(c as u8 - 97) as usize] += 1;
            }
        }
    }
    let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();
    for (i, freq_arr) in freq.iter().enumerate() {
        map.entry(*freq_arr)
            .and_modify(|vec| vec.push(strs[i].clone()))
            .or_insert(vec![strs[i].clone()]);
    }
    let res: Vec<Vec<String>> = map.into_values().collect();
    res
}
fn main() {
    let strs: Vec<String> = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    println!("{:?}", group_anagrams(strs));
}
