pub fn is_valid(s: String) -> bool {
    let par: HashMap<char, char> = HashMap::from([
        (')', '('), 
        (']', '['), 
        ('}', '{')
    ]);
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {

        if let Some(&expected_paren) = par.get(&c) {

            if stack.pop() != Some(expected_paren) {
                return false;
            }

        } else {
            stack.push(c);
        }
    }

    stack.len() == 0
}
