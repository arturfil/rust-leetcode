// better version of anagram
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false; }
    let mut letters = [0i32; 26];

    for i in s.chars() {
        let val = i as usize - 'a' as usize;
        letters[val] += 1;
    }

    for i in t.chars() {
        let val = i as usize - 'a' as usize;
        letters[val] -= 1;

        if letters[val] < 0 { return false; }
    }

    true
}
