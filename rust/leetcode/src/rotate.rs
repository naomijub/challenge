pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize;

    nums.rotate_right(k % len)
}

pub fn rotate_reversing(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();

    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

#[test]
fn test_1() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;

    rotate(&mut v, k);

    assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4,])
}

#[test]
fn test_2() {
    let mut v = vec![-1, -100, 3, 99];
    let k = 2;

    rotate(&mut v, k);

    assert_eq!(v, vec![3, 99, -1, -100])
}

#[test]
fn test_3() {
    let mut v = vec![1, 2];
    let k = 3;

    rotate(&mut v, k);

    assert_eq!(v, vec![2, 1])
}
