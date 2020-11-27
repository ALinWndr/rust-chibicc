pub fn strtol(s: &String) -> (Option<u64>, String) {
    if s.is_empty() {
        return (None, s.clone());
    }

    let mut pos = 0;
    let mut remaining = s.clone();
    let len = s.len();

    while len != pos {
        if !s.chars().nth(pos).unwrap().is_ascii_digit() {
            break;
        }
        pos += 1;
    }

    if len == pos {
        (Some(remaining.parse::<u64>().unwrap()), "".into())
    } else {
        let t: String = remaining.drain(..pos).collect();
        (Some(t.parse::<u64>().unwrap()), remaining)
    }
}
