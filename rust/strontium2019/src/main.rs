use std::collections::HashSet;
fn main(){
    let mut set: HashSet<String> = HashSet::new();
    let mut array = vec!["xxbxx", "xbx", "x"];
    heap_alg(array.len(), &mut array, &mut set);

    println!("{:?}", set);
    println!("{:?}", set.into_iter().map(|s| max_repeating_count(s)).max().unwrap());
}

fn heap_alg(n : usize, a : &mut Vec<&str>, permutations: &mut HashSet<String>) {
    if n == 1 {
        permutations.insert(a.join(""));
    }
    else {
        for i in  0 .. n - 1 {
            heap_alg(n - 1, a, permutations);

            if n % 2 == 0 {
                a.swap(i, n - 1);
            }
            else {
                a.swap(0, n - 1);
            }
        }
        heap_alg(n - 1, a, permutations);
    }
}

fn max_repeating_count(s: String) -> i32 
{ 
        let len = s.len(); 
        let mut count = 0; 
  
        for i in 0..len {
            let mut cur_count = 1;
            for j in i+1..len {
                if &s[i..i+1] != &s[j..j+1] {break;}
                cur_count += 1; 
            }

            if cur_count > count {
                count = cur_count;

            }
        }

        count
    }