use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, sync::mpsc::channel};
use threadpool::ThreadPool;

fn main() {
    let f = OpenOptions::new().read(true).open("words.txt").unwrap();
    let buf = BufReader::new(f);
    let lines = buf.lines();

    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    for line in lines {
        if let Ok(line) = line {
            let tx = tx.clone();
            let line_aux = line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            pool.execute(move || {
                let words = line_aux;
                let count = words.into_iter().map(|word| (word, 1usize)).fold(
                    HashMap::new(),
                    |mut acc, (word, count)| {
                        let hm = acc.entry(word).or_insert(0usize);
                        *hm += count;

                        acc
                    },
                );
                tx.send(count)
                    .expect("channel will be there waiting for the pool");
            });
        }
    }

    let result = rx
        .iter()
        .filter(|x| !x.is_empty())
        .take(6)
        .fold(HashMap::new(), |mut acc, hm| {
            for (k, v) in hm {
                let m = acc.entry(k).or_insert(0usize);
                *m += v;
            }
            acc
        });

    assert_eq!(result.get(&String::from("words")), Some(&5));
    println!("{:?}", result);
}
