#![feature(is_sorted)]

pub fn alpha(a: Vec<usize>) -> Option<usize> {
    if a.is_empty(){
        None
    } else  if a.is_sorted() || a.is_sorted_by(|a, b| b.partial_cmp(&a)) {
        Some(a.len() - 1)
    } else {
        Some(a.iter().enumerate()
            .take_while(|(idx, val)| **val <= *a.get(idx.saturating_sub(1)).unwrap_or(&usize::MAX))
            .count()
            .saturating_sub(1))
    }
}

#[cfg(test)]
mod tests {
    use super::alpha;
    #[test]
    fn empty_vec_returns_none() {
        let actual = alpha(vec![]);
        
        assert_eq!(actual, None);
    }

    #[test]
    fn vec_is_sorted() {
        let a = vec![0, 1, 2];
        let actual = alpha(a);

        assert_eq!(actual, Some(2));
    }

    #[test]
    fn vec_is_reverser_sorted() {
        let a = vec![2, 1, 0];
        let actual = alpha(a);

        assert_eq!(actual, Some(2));
    }

    #[test]
    fn hard_one() {
        let a = vec![2, 2, 1, 0, 1];
        let actual = alpha(a);

        assert_eq!(actual, Some(3));
    }
}
