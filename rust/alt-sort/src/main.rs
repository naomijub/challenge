fn main() {
    let v = vec![2, 1, 4, 3, 5, 6, 7, 10, 9, 8];
    let sum: i32 = v.iter().sum();
    let mean = sum as f64 / v.len() as f64;
    
    let mut alt = Vec::new();
    let mut lesser = Vec::new();
    let mut greater = Vec::new();

    v.into_iter().for_each(|e| {
        if e as f64 > mean {
            greater.push(e);
        } else  {
            lesser.push(e);
        }
    });

    if lesser.len() > greater.len() {
        let max = lesser.iter().max().unwrap();
        alt.push(*max);
        greater.push(*max);
    } else if lesser.len() < greater.len() {
        let min = greater.iter().min().unwrap();
        alt.push(*min);
        lesser.push(*min);
    }

    let zip = lesser.into_iter().zip(greater.into_iter());
    for (eg, el) in zip {
        if !alt.contains(&eg) {
            alt.push(eg);
        }
        if !alt.contains(&el) {
            alt.push(el);
        }
    }

    println!("{:?}", alt)
}