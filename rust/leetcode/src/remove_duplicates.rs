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

pub fn remove_duplicates_inplace(nums: &mut Vec<i32>) -> i32 {
    let mut last = i32::MIN;
    let aux = nums.clone();

    let index_iter = aux.iter().enumerate().filter_map(|(i, num)| {
        if *num == last {
            Some(i)
        } else {
            last = *num;
            None
        }
    });

    for i in index_iter.rev() {
        nums.remove(i);
    }

    nums.len() as i32
}

pub fn remove_duplicates_inplace_proc(nums: &mut Vec<i32>) -> i32 {
    let mut last = i32::MIN;

    for i in (0..nums.len()).rev() {
        if nums[i] == last {
            nums.remove(i);
        } else {
            last = nums[i];
        }
    }

    nums.len() as i32
}

pub fn remove_double_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut current_nums = [i32::MIN; 2];
    let mut to_remove = Vec::new();

    for (idx, num) in nums.iter().enumerate() {
        match (current_nums.get(0), current_nums.get(1)) {
            (None, None) => {
                current_nums[0] = *num;
            }
            (None, Some(_)) => panic!("None, Some"),
            (Some(same), None) if same == num => {
                current_nums[1] = *num;
            }

            (Some(same1), Some(same2)) if same1 == same2 && same1 == num => {
                to_remove.push(idx);
            }
            (Some(other), Some(same)) if same != other && same == num => {
                current_nums[0] = *num;
            }
            (Some(same), Some(other)) if same != other && same == num => panic!(),
            _ => {
                current_nums[1] = *num;
            }
        }
    }

    to_remove.into_iter().rev().for_each(|i| {
        nums.remove(i);
    });
    nums.len() as i32
}

pub fn remove_double_duplicates_builtin(nums: &mut Vec<i32>) -> i32 {
    let mut last_2 = (i32::MIN, i32::MIN);

    nums.retain(|num| {
        let result = last_2.0 == last_2.1 && last_2.1 == *num;

        last_2.0 = last_2.1;
        last_2.1 = *num;

        !result
    });

    nums.len() as i32
}

#[test]
fn test_double_1() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(remove_double_duplicates(&mut nums), 5);
    assert_eq!(nums, vec![1, 1, 2, 2, 3]);
}

#[test]
fn test_double_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    assert_eq!(remove_double_duplicates(&mut nums), 7);
    assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
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
