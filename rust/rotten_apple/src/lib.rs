pub type Apple = u32;

fn black_box_balance_item(w1: Apple, w2: Apple) -> i8 {
    match (w1, w2) {
        (a, b) if a == b => 0,
        (a, b) if a > b => 1,
        (a, b) if a < b => -1,
        _ => unreachable!("{} _ {}", w1, w2),
    }
}

/// Find rotten apple among a set of Apples.
/// The only thing you are certain is that the rotten apple has a different weight that the normal apples
/// And the only way to measure this is by comparing two apples with the black_box_balance_item function
pub fn find_rotten_apple(apples: &[Apple]) -> Option<usize> {
    if apples.is_empty() {
        return None;
    } else if apples.len() == 1 {
        return Some(0);
    } else if apples.len() == 2 && black_box_balance_item(apples[0], apples[1]) == 1 {
        return Some(0);
    } else if apples.len() == 2 && black_box_balance_item(apples[0], apples[1]) == -1 {
        return Some(1);
    }

    let apples: Vec<(usize, Apple)> = apples.iter().enumerate().map(|(i, a)| (i, *a)).collect();
    let filtered_apples = if apples.len() > 3 {
        apples
            .windows(3)
            .filter(|items| {
                black_box_balance_item(items[0].1, items[1].1)
                    != black_box_balance_item(items[2].1, items[1].1)
            })
            .last()?
    } else {
        apples.as_slice()
    };

    Some(
        if black_box_balance_item(filtered_apples[0].1, filtered_apples[1].1) == 0 {
            filtered_apples[2].0
        } else if black_box_balance_item(filtered_apples[0].1, filtered_apples[2].1) == 0 {
            filtered_apples[1].0
        } else {
            filtered_apples[0].0
        },
    )
}

#[test]
fn large_odd_set() {
    let apples = vec![2, 2, 2, 2, 3, 2, 2, 2, 2];

    assert_eq!(Some(4), find_rotten_apple(&apples));
}

#[test]
fn large_even_set() {
    let apples = vec![2, 2, 2, 2, 2, 3, 2, 2, 2, 2];

    assert_eq!(Some(5), find_rotten_apple(&apples));
}

#[test]
fn size_3_odd_set() {
    let apples = vec![2, 3, 2];

    assert_eq!(Some(1), find_rotten_apple(&apples));
}

#[test]
fn empty_set() {
    let apples = vec![];

    assert_eq!(None, find_rotten_apple(&apples));
}

#[test]
fn one_apple_set() {
    let apples = vec![1];

    assert_eq!(Some(0), find_rotten_apple(&apples));
}

#[test]
fn size_2_last_pos_set() {
    let apples = vec![2, 3];

    assert_eq!(Some(1), find_rotten_apple(&apples));
}

#[test]
fn size_2_first_pos_set() {
    let apples = vec![3, 2];

    assert_eq!(Some(0), find_rotten_apple(&apples));
}
