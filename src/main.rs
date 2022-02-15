use std::collections::HashMap;
use std::io::{self, prelude::*};

fn main() {
    let _stdin = io::stdin();
    let mut stdin = _stdin.lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s);

    let mut lines = s.lines();
    lines.next();

    let mut people = Vec::new();

    // let mut ingredients = HashMap::new();

    let mut pizza = Vec::new();

    loop {
        let liked: Vec<_> = match lines.next() {
            Some(x) => x.split_whitespace().skip(1).collect(),
            _ => break,
        };
        let disliked: Vec<_> = lines.next().unwrap().split_whitespace().skip(1).collect();

        people.push((liked, disliked));
    }

    
    loop {
        let mut like_freq = HashMap::new();

        for (liked, disliked) in &people {
            for l in liked {
                *(&mut like_freq.entry(*l).or_insert((0, 0)).0) += 1;
            }

            for d in disliked {
                *(&mut like_freq.entry(*d).or_insert((0, 0)).1) += 1;
            }
        }

        let max = match like_freq.iter().max_by_key(|(_, (a, b))| a - b) {
            Some(x) => x,
            None => break,
        };

        if max.1.0 - max.1.1 <= 0 {
            break;
        }

        pizza.push(max.0.to_string());
        
        people.retain(|p| !p.1.contains(max.0));
    }

    println!("{:?}", pizza);

    println!("Hello, world!");
}
