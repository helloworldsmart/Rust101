fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut m: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

        if s.len() != t.len() {
            return false;
        }

        s.chars().zip(t.chars()).for_each(|(s, t)| {
            if s == t {
                return;
            }

            m.entry(s).and_modify(|e| {*e += 1}).or_insert(1);
            m.entry(t).and_modify(|e| {*e -= 1}).or_insert(-1);
        });
        m.values().find(|&&e| e != 0).is_none()

    }
}