use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    let mut mappings = HashMap::new();
    mappings.insert(')', '(');
    mappings.insert('}', '{');
    mappings.insert(']', '[');

    let chars = s.chars();
    for c in chars {
        if mappings.contains_key(&c) {
            let mut top_element = '#';
            if !stack.is_empty() {
                top_element = stack.pop().unwrap();
            }
            if top_element != *mappings.get(&c).unwrap() {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}

#[test]
fn test_is_valid() {
    let parentheses1 = "()".to_string();
    let parentheses2 = "()[]{}".to_string();
    let parentheses3 = "(]".to_string();
    assert_eq!(true, is_valid(parentheses1));
    assert_eq!(true, is_valid(parentheses2));
    assert_eq!(false, is_valid(parentheses3));
}
