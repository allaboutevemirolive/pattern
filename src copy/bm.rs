pub fn boyer_moore_search(text: &str, pattern: &str) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        return Some(0);
    }

    let mut bad_char = vec![m; 256];

    for (i, &c) in pattern.as_bytes().iter().enumerate().take(m - 1) {
        bad_char[c as usize] = (m - 1 - i) as usize;
    }

    let mut i = m - 1;
    let mut j = m - 1;

    while i < n {
        if pattern.as_bytes()[j] == text.as_bytes()[i] {
            if j == 0 {
                return Some(i);
            }
            i -= 1;
            j -= 1;
        } else {
            i += bad_char[text.as_bytes()[i] as usize].max(1) as usize;
            j = m - 1;
        }
    }

    None
}
