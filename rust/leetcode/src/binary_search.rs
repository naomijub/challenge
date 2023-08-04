pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(idx) => idx as i32,
        Err(_) => -1,
    }
}

#[test]
fn test_1() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(search(nums, target), 4);
}

#[test]
fn test_2() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    assert_eq!(search(nums, target), -1);
}
