fn main() {
    let unordered_logs = vec![
        "[a3 dec fgh tre]".to_string(),
        "[a1 dec fgh tre]".to_string(),
        "[b2 1 43 54]".to_string(),
        "[a4 abf edr ter]".to_string(),
        "[r4 3 45 2]".to_string()
    ];

    println!("{:?}", order_logs(unordered_logs));
}

fn order_logs(logs: Vec<String>) -> Vec<String> {
    let mut parts = logs.iter().map(|l| l.split_at(3)).collect::<Vec<(&str,&str)>>();
    parts.sort_by(|a, b| {
        match (a.1.find(char::is_alphabetic), b.1.find(char::is_alphabetic)) {
            (Some(_), Some(_)) => a.1.cmp(b.1).then(a.0.cmp(b.0)),
            (Some(_), None) => b.1.cmp(a.1).then(a.0.cmp(b.0)),
            (None, Some(_)) => b.1.cmp(a.1).then(a.0.cmp(b.0)),
            (None, None) => a.1.cmp(b.1).then(a.0.cmp(b.0)),
        }
    });
    
    parts.iter().map(|l| String::from(l.0) + l.1).collect()
}


#[test]
fn order_string() {
    let unordered_logs = vec![
        "[a3 dec fgh tre]".to_string(),
        "[a1 dec fgh tre]".to_string(),
        "[b2 1 43 54]".to_string(),
        "[a4 abf edr ter]".to_string(),
        "[r4 3 45 2]".to_string()
    ];
    let ordered_logs = vec![
        "[a4 abf edr ter]".to_string(),
        "[a1 dec fgh tre]".to_string(),
        "[a3 dec fgh tre]".to_string(),
        "[b2 1 43 54]".to_string(),
        "[r4 3 45 2]".to_string()
    ];

    assert_eq!(ordered_logs, order_logs(unordered_logs));
}

#[test]
fn empty_logs() {
    let empty_list: Vec<String> = Vec::new();
    let empty_expect: Vec<String> = Vec::new();
    
    assert_eq!(empty_expect, order_logs(empty_list));
}