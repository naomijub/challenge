// At least iot is fast?
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut elements = std::collections::BTreeSet::new();
    for i in &*nums {
        elements.insert(*i);
    }

    let len = elements.len();
    *nums = elements.into_iter().collect();
    len as i32
}

#[test]
fn test_1() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums), 2);
    assert_eq!(nums, vec![1, 2]);
}

#[test]
fn test_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums), 5);
    assert_eq!(nums, vec![0, 1, 2, 3, 4]);
}
