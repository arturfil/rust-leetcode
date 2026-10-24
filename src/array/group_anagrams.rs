use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut seen: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for str in strs {
        let mut k_srt: Vec<char> = str.chars().collect();
        k_srt.sort();
        seen.entry(k_srt).or_insert(vec![]).push(str);
    }

    seen.values().cloned().collect()
}

/*

group_anagrams(vec![
    "ate".to_string(),
    "tae".to_string(),
    "tea".to_string(),
    "eat".to_string(),
    "tar".to_string(),
    "rat".to_string(),
    "pat".to_string(),
]);

*/
