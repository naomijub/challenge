pub fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let size = s.len() / 2;
    let iter = s.chars().filter(|c| c.is_alphanumeric());
    let rev = iter.clone().rev();
    iter.zip(rev).take(size).all(|(a, b)| a == b)
}

#[test]
fn panama() {
    let s = "A man, a plan, a canal: Panama";
    assert!(is_palindrome(s));
}
