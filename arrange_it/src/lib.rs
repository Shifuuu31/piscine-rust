pub fn arrange_phrase(phrase: &str) -> String {
    let mut words = vec!["".to_string(); phrase.split_whitespace().count()];

    for s in phrase.split_whitespace() {
        if let Some((i, c)) = s.char_indices().find(|(_, c)| c.is_ascii_digit()) {
            let idx = c.to_digit(10).unwrap() as usize - 1; // make it 0-based
            words[idx] =  s[..i].to_string() + &s[i + 1..];        }
    }

    words.join(" ")
}
