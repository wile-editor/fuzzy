use std::ascii::AsciiExt;

pub const BONUS_FIRST: Score = 2;
pub const BONUS_UPPERCASE: Score = 1;
pub const BONUS_AFTER_DELIM: Score = 3;

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

    let mut bonuses = [0; 1024];
    bonuses[0] = BONUS_FIRST;

    let mut next_bonus = 0;
    for (i, &h) in haystack.iter().enumerate() {
        let (now, next) = match h {
            b'A' ... b'Z' => (next_bonus + BONUS_UPPERCASE, 0),
            b'-' | b'.' | b'/' | b':' | b'_' => (next_bonus, BONUS_AFTER_DELIM),
            _ => (next_bonus, 0)
        };
        bonuses[i] += now;
        next_bonus = next;
    }

    let mut start_at = 0;
    for &n in needle.iter() {
        let mut found = false;
        for i in range(start_at, haystack.len()) {
            if n.eq_ignore_ascii_case(&haystack[i]) {
                if !found {
                    start_at = i + 1;
                    found = true;
                }
                curr[i+1] = std::cmp::max(prev[i] + 1 + bonuses[i], curr[i]);
            } else {
                curr[i+1] = curr[i];
            }
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[haystack.len()]
}

pub fn sorted<'a, 'tmp>(haystacks: &'a [&[u8]],
                        needle: &'tmp [u8]) -> Vec<&'a [u8]> {
    use std::cmp::Ordering::{Less, Equal, Greater};
    let mut ret = haystacks.iter().map(|&h| h).collect::<Vec<_>>();
    ret.sort_by(|&a, &b| {
        match score(a, needle).cmp(&score(b, needle)) {
            Greater => Less,
            Equal => a.len().cmp(&b.len()),
            Less => Greater
        }
    });
    ret
}

fn is_subsequence_of(smaller: &[u8], larger: &[u8]) -> bool {
    let mut larger = larger.iter();

    for s in smaller.iter() {
        loop {
            match larger.next() {
                Some(l) if s.eq_ignore_ascii_case(l) => break,
                Some(_)                              => {},
                None                                 => return false
            }
        }
    }

    true
}
