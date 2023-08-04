pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((i32::MAX, 0), |(min, max_profit), &value| {
            (value.min(min), max_profit.max(value - min))
        })
        .1
}

#[test]
fn test_1() {
    let nums = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(nums), 5);
}

#[test]
fn test_2() {
    let nums = vec![7, 6, 4, 3, 1];
    assert_eq!(max_profit(nums), 0);
}

#[test]
fn test_3() {
    let nums = vec![2, 5, 1, 3, 1];
    assert_eq!(max_profit(nums), 3);
}

#[test]
fn test_4() {
    let nums = vec![2, 4, 1];
    assert_eq!(max_profit(nums), 2);
}

#[test]
fn test_5() {
    let nums = vec![1, 2];
    assert_eq!(max_profit(nums), 1);
}

#[test]
fn test_6() {
    let nums = vec![1, 2, 4];
    assert_eq!(max_profit(nums), 3);
}
