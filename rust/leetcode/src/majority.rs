pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::<i32, i32>::new();

    for num in nums {
        let map = map.entry(num).or_insert(1);
        *map += 1;
    }

    map.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
}

#[test]
fn test_1() {
    let vec = vec![3, 2, 3];

    assert_eq!(majority_element(vec), 3);
}

#[test]
fn test_2() {
    let vec = vec![2, 2, 1, 1, 1, 2, 2];

    assert_eq!(majority_element(vec), 2);
}
