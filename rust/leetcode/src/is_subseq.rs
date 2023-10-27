pub fn is_subsequence(s: &str, t: &str) -> bool {
    s.chars()
        .try_fold(t.chars(), |mut acc, x| acc.find(|&y| x == y).map(|_| acc))
        .is_some()
}

pub fn is_subsequence2(s: &str, t: &str) -> bool {
    if t.is_empty() && !s.is_empty() {
        return false;
    }

    if s.is_empty() {
        return true;
    }

    let s = s.to_lowercase();
    let t = t.to_lowercase();
    let mut s_iter = s.chars();
    let mut t_iter = t.chars();

    consume(s_iter.next(), &mut s_iter, &mut t_iter)
}

fn consume(next: Option<char>, s: &mut std::str::Chars<'_>, t: &mut std::str::Chars<'_>) -> bool {
    if let Some(nx) = next {
        if let Some(t_next) = t.next() {
            if nx == t_next {
                let aux_next = s.next();
                consume(aux_next,  s, t)
            } else {
                consume(next, s, t)
            }
        } else {
            false
        }
    } else {
        true
    }
}

#[test]
fn test_1() {
    let s = "abc";
    let t = "ahbgdc";
    assert!(is_subsequence(s, t));
}

#[test]
fn test_2() {
    let s = "axc";
    let t = "ahbgdc";
    assert!(!is_subsequence(s, t));
}

#[test]
fn test_3() {
    let s = "ace";
    let t = "abcde";
    assert!(is_subsequence(s, t));
}

#[test]
fn test_4() {
    let s = "aec";
    let t = "abcde";
    assert!(!is_subsequence(s, t));
}
