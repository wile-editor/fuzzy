pub fn score(haystack: &[u8], needle: &[u8]) -> u16 {
    if needle.len() == 0 {
        return 0
    }

    if !is_subsequence_of(needle, haystack) {
        return 0
    }

    unimplemented!()
}

fn is_subsequence_of(smaller: &[u8], larger: &[u8]) -> bool {
    let mut larger = larger.iter();

    for s in smaller.iter() {
        loop {
            match larger.next() {
                Some(l) if s == l => break,
                Some(_)           => {},
                None              => return false
            }
        }
    }

    true
}
