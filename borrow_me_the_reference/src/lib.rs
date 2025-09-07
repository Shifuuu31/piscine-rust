pub fn delete_and_backspace(s: &mut String) {
    let mut result: String = String::new();
    let mut skip_count: i32 = 0;

    for c in s.chars() {
        match c {
            '-' => {
                result.pop();
            }
            '+' => {
                skip_count += 1;
            }
            _ => {
                if skip_count > 0 {
                    skip_count -= 1;
                } else {
                    result.push(c);
                }
            }
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if
            let Some((idx, op)) = s
                .char_indices()
                .find(|(_, c)| (*c == '+' || *c == '-' || *c == '*' || *c == '/'))
        {
            *s = match op {
                '+' =>
                    (
                        s[..idx].parse::<i64>().unwrap() + s[idx + 1..].parse::<i64>().unwrap()
                    ).to_string(),
                '-' =>
                    (
                        s[..idx].parse::<i64>().unwrap() - s[idx + 1..].parse::<i64>().unwrap()
                    ).to_string(),
                '*' =>
                    (
                        s[..idx].parse::<i64>().unwrap() * s[idx + 1..].parse::<i64>().unwrap()
                    ).to_string(),
                '/' =>
                    (
                        s[..idx].parse::<i64>().unwrap() / s[idx + 1..].parse::<i64>().unwrap()
                    ).to_string(),
                _ => unreachable!(),
            };
        }
    }
}
