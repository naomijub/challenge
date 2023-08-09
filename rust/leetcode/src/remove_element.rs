pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| x != &val);
    nums.len() as i32
}

pub fn remove_element_fp(nums: &mut Vec<i32>, val: i32) -> i32 {
    (0..nums.len())
        .fold((0, 0), |(count, index_to_swap), i| {
            if nums[i] == val {
                (count, index_to_swap + 1)
            } else {
                nums[i - index_to_swap] = nums[i];
                (count + 1, index_to_swap)
            }
        })
        .0
}

pub fn remove_element_for(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();
    let mut current_last = len;
    if nums.is_empty() {
        return 0;
    }

    for i in (0..len).rev() {
        if nums[i] == val && current_last != len {
            nums.swap(i, current_last - 1);
            current_last = current_last.saturating_sub(1);
        } else if nums[i] == val && current_last == len {
            nums.swap(i, current_last - 1);
            current_last = current_last.saturating_sub(1);
        }
    }

    nums.resize(current_last, 0);
    nums.len() as i32
}

#[test]
fn test_1() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(remove_element_for(&mut nums, 3), 2);
    assert_eq!(nums, vec![2, 2])
}

#[test]
fn test_2() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element_for(&mut nums, 2), 5);
    assert_eq!(nums, vec![0, 1, 0, 4, 3])
}

#[test]
fn test_3() {
    let mut nums = vec![1, 1];
    assert_eq!(remove_element_for(&mut nums, 1), 0);
    assert_eq!(nums, vec![])
}

#[test]
fn test_4() {
    let mut nums = vec![4, 5];
    assert_eq!(remove_element_for(&mut nums, 4), 1);
    assert_eq!(nums, vec![5])
}
