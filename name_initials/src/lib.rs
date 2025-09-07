pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let mut parts = name.split(' ');
            let first = parts.next().expect("wrong format")[0..1].to_string() + ".";
            let last = parts.next().expect("wrong format")[0..1].to_string() + ".";
            first + " " + &last
        })
        .collect::<Vec<String>>()
}
