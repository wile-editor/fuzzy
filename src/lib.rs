pub type Score = u16;

pub fn score(haystack: &[u8], needle: &[u8]) -> Score {
    if needle.len() == 0 {
        return 0
    }

    if !is_subsequence_of(needle, haystack) {
        return 0
    }

    let mut prev = [0; 1024];
    let mut curr = [0; 1024];

    let mut start_at = 0;
    for &n in needle.iter() {
        let mut found = false;
        for i in range(start_at, haystack.len()) {
            if n == haystack[i] {
                if !found {
                    start_at = i + 1;
                    found = true;
                }
                curr[i+1] = std::cmp::max(prev[i] + 1, curr[i]);
            } else {
                curr[i+1] = curr[i];
            }
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[haystack.len()]
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
