pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let _ = nums1.splice(m.., nums2.to_owned().into_iter().take(n as usize)).collect::<Vec<i32>>();
    nums1.sort();
}


#[test]
fn test_1() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let m = 3;
    let mut nums2 = vec![2,5,6];
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    let expected = vec![1,2,2,3,5,6];
    assert_eq!(nums1, expected);
}

#[test]
fn test_4() {
    let mut nums1 = vec![0,0,0,0,0];
    let m = 3;
    let mut nums2 = vec![1,2,3,4,5];
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    let expected = vec![1,2,2,3,5];
    assert_eq!(nums1, expected);
}

#[test]
fn test_2() {
    let mut nums1 = vec![1];
    let m = 1;
    let mut nums2 = vec![];
    let n = 0;

    merge(&mut nums1, m, &mut nums2, n);

    let expected = vec![1];
    assert_eq!(nums1, expected);
}

#[test]
fn test_3() {
    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;

    merge(&mut nums1, m, &mut nums2, n);

    let expected = vec![1];
    assert_eq!(nums1, expected);
}