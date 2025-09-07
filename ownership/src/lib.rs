pub fn first_subword(s: String) -> String {
    for (i, c) in s[1..].chars().enumerate() {
        if !c.is_ascii_lowercase() {
            return s[..=i].to_string();
        }
    }
    s
}
