#[allow(dead_code)]
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

#[allow(dead_code)]
pub fn contain_forbidden_characters(s: &str) -> bool {
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\'];
    s.chars().any(|c| forbidden_characters.contains(&c))
}
