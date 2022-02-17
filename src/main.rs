use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{stdin, Read},
};

use signal_hook::{consts::SIGINT, iterator::Signals};

#[derive(Debug, Clone)]
pub struct Ingredient<'i> {
    name: &'i str,
    likers: Vec<usize>,
    haters: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Input<'i> {
    ings: Vec<Ingredient<'i>>,
    clients: Vec<[Vec<usize>; 2]>,
}

type Pizza = HashSet<usize>;

impl<'i> Input<'i> {
    pub fn parse(s: &'i String) -> Self {
        let mut lines = s.lines();

        let c: usize = lines.next().unwrap().parse().unwrap();
        // const C: usize = 100_000;
        let mut index_of = HashMap::<&'i str, usize>::new();
        let mut ings: Vec<Ingredient<'i>> = vec![];
        let mut index = 0;
        let clients: Vec<[Vec<usize>; 2]> = (0..c)
            .map(|c| {
                [
                    lines
                        .next()
                        .unwrap()
                        .split(' ')
                        .skip(1)
                        .map(|like| {
                            if let Some(&i) = index_of.get(&like) {
                                ings[i].likers.push(c);
                                i
                            } else {
                                index_of.insert(like, index);
                                assert!(ings.len() == index);
                                ings.push(Ingredient {
                                    name: like,
                                    likers: vec![c],
                                    haters: vec![],
                                });
                                index += 1;
                                index - 1
                            }
                        })
                        .collect(),
                    lines
                        .next()
                        .unwrap()
                        .split(' ')
                        .skip(1)
                        .map(|dislike| {
                            if let Some(&i) = index_of.get(&dislike) {
                                ings[i].haters.push(c);
                                i
                            } else {
                                index_of.insert(dislike, index);
                                assert!(ings.len() == index);
                                ings.push(Ingredient {
                                    name: dislike,
                                    likers: vec![],
                                    haters: vec![c],
                                });
                                index += 1;
                                index - 1
                            }
                        })
                        .collect(),
                ]
            })
            .collect();
        Self { ings, clients }
    }
}

fn parse_pizza(input: &Input, pizza: &str) -> Pizza {
    let mut res = Pizza::new();
    for i in pizza.split(' ').skip(1) {
        for (index, ing) in input.ings.iter().enumerate() {
            if ing.name == i {
                res.insert(index);
            }
        }
    }
    res
}

fn bake_pizza<'i>(input: &Input<'i>) -> Pizza {
    let mut pizza = HashSet::new();
    let mut scores: Vec<isize> = input
        .ings
        .iter()
        .map(|i| i.likers.len() as isize - i.haters.len() as isize)
        .collect();
    let mut dropped = vec![false; input.clients.len()];

    loop {
        let ing = match scores
            .iter()
            .enumerate()
            .filter(|(i, _)| !pizza.contains(i))
            .max_by_key(|&(_, s)| s)
        {
            Some((i, &s)) if s >= 0 => i,
            _ => break pizza,
        };
        pizza.insert(ing);
        for &hater in &input.ings[ing].haters {
            if !dropped[hater] {
                dropped[hater] = true;
                for &i in &input.clients[hater][0] {
                    scores[i] -= 1;
                }
                for &i in &input.clients[hater][1] {
                    scores[i] += 1;
                }
            }
        }
    }
}

fn do_change(
    input: &Input,
    pizza: &mut Pizza,
    happy: &mut [bool],
    i: usize,
) -> (Vec<(usize, bool)>, isize) {
    let mut toggled = vec![];
    let mut delta = 0isize;
    if pizza.insert(i) {
        for &liker in &input.ings[i].likers {
            if happy[liker] {
                continue;
            }
            let mut nvm = false;
            for like in &input.clients[liker][0] {
                if !pizza.contains(like) {
                    nvm = true;
                    break;
                }
            }
            for dislike in &input.clients[liker][1] {
                if pizza.contains(dislike) {
                    nvm = true;
                    break;
                }
            }
            if !nvm {
                // eprintln!("gaining liker: {}", liker,);
                happy[liker] = true;
                toggled.push((liker, false));
                delta += 1;
            }
        }
        for &hater in &input.ings[i].haters {
            if !happy[hater] {
                continue;
            }
            // eprintln!("losing hater: {}", hater,);
            happy[hater] = false;
            toggled.push((hater, true));
            delta -= 1;
        }
    } else {
        // eprintln!("removing {}", ing.name);
        pizza.remove(&i);

        for &hater in &input.ings[i].haters {
            if happy[hater] {
                continue;
            }
            let mut nvm = false;
            for like in &input.clients[hater][0] {
                if !pizza.contains(like) {
                    nvm = true;
                    break;
                }
            }
            for dislike in &input.clients[hater][1] {
                if pizza.contains(dislike) {
                    nvm = true;
                    break;
                }
            }
            if !nvm {
                // eprintln!("gaining hater: {}", hater,);
                happy[hater] = true;
                toggled.push((hater, false));
                delta += 1;
            }
        }
        for &liker in &input.ings[i].likers {
            if !happy[liker] {
                continue;
            }
            // eprintln!("losing liker: {}", liker,);
            happy[liker] = false;
            toggled.push((liker, true));
            delta -= 1;
        }
    }
    (toggled, delta)
}

fn spice_pizza(input: &Input, pizza: &mut Pizza, happy: &mut [bool]) -> isize {
    let mut best_delta = isize::MIN;
    let mut best_ing = (0, 0, 0);
    let indices = |n| {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut is = (0..n).collect::<Vec<usize>>();
        is.shuffle(&mut thread_rng());
        is
    };
    for i in indices(input.ings.len()) {
        let (t1, d1) = do_change(input, pizza, happy, i);
        for j in indices(input.ings.len()) {
            if i == j {
                continue;
            }
            let (t2, d2) = do_change(input, pizza, happy, j);
            for k in indices(input.ings.len()) {
                if i == k || j == k {
                    continue;
                }
                let (t3, d3) = do_change(input, pizza, happy, k);

                let new_delta = d1 + d2 + d3;
                if new_delta > best_delta || new_delta == best_delta && rand::random() {
                    best_delta = new_delta;
                    best_ing = (i, j, k);
                }

                if !pizza.insert(k) {
                    pizza.remove(&k);
                }
                for (t, v) in t3 {
                    happy[t] = v;
                }
            }

            if !pizza.insert(j) {
                pizza.remove(&j);
            }
            for (t, v) in t2 {
                happy[t] = v;
            }
        }
        if !pizza.insert(i) {
            pizza.remove(&i);
        }
        for (t, v) in t1 {
            happy[t] = v;
        }
    }
    let (_, d1) = do_change(input, pizza, happy, best_ing.0);
    let (_, d2) = do_change(input, pizza, happy, best_ing.1);
    let (_, d3) = do_change(input, pizza, happy, best_ing.2);
    assert_eq!(best_delta, d1 + d2 + d3);

    d1 + d2
}

fn krydda_pizza(input: &Input, pizza: &mut Pizza, happy: &mut Vec<bool>) -> isize {
    let mut best_score = isize::MIN;
    let mut best_ing: isize = -1;
    let mut dont_hate_it: Vec<bool> = vec![true; input.clients.len()];
    for ingredient in &*pizza {
        for (p_idx, p) in dont_hate_it.iter_mut().enumerate() {
            if input.clients[p_idx][1].contains(ingredient) {
                *p = false;
            }
        }
    }

    for (i, _ing) in input
        .ings
        .iter()
        .enumerate()
        .filter(|(i, _)| !pizza.contains(i))
    {
        let (base, score) = improvability::score(input, &*pizza, &dont_hate_it[..], i);

        if base > 0 && score > best_score {
            best_score = score;
            best_ing = i as isize;
        }
    }
    if best_ing < 0 {
        0
    } else {
        let (_, delta) = do_change(input, pizza, happy, best_ing as usize);

        delta
    }
}

mod improvability;

fn season_pizza(input: &Input, pizza: &mut Pizza, happy: &mut [bool]) -> isize {
    let mut total_delta = 0;

    let mut best_score = (0, isize::MIN);

    for (i, ing) in input
        .ings
        .iter()
        .enumerate()
        .filter(|(x, _)| !pizza.contains(x))
    {
        let (base, score) = improvability::score(input, &pizza, &happy[..], i);
        if best_score.1 < score {
            best_score = (i, score);
        }
    }

    for (i, ing) in input.ings.iter().enumerate() {
        if rand::random::<usize>() < usize::MAX / 2 {
            continue;
        }
        let mut delta = 0isize;
        let mut toggled = vec![];
        // let pre_score = taste_pizza(&input, &pizza) as isize;
        if pizza.insert(i) {
            // eprintln!("adding {}", ing.name);

            for &liker in &ing.likers {
                if happy[liker] {
                    continue;
                }
                let mut nvm = false;
                for like in &input.clients[liker][0] {
                    if !pizza.contains(like) {
                        nvm = true;
                        break;
                    }
                }
                for dislike in &input.clients[liker][1] {
                    if pizza.contains(dislike) {
                        nvm = true;
                        break;
                    }
                }
                if !nvm {
                    // eprintln!("gaining liker: {}", liker,);
                    happy[liker] = true;
                    toggled.push((liker, false));
                    delta += 1;
                }
            }
            for &hater in &ing.haters {
                if !happy[hater] {
                    continue;
                }
                // eprintln!("losing hater: {}", hater,);
                happy[hater] = false;
                toggled.push((hater, true));
                delta -= 1;
            }
        } else {
            // eprintln!("removing {}", ing.name);
            pizza.remove(&i);

            for &hater in &ing.haters {
                if happy[hater] {
                    continue;
                }
                let mut nvm = false;
                for like in &input.clients[hater][0] {
                    if !pizza.contains(like) {
                        nvm = true;
                        break;
                    }
                }
                for dislike in &input.clients[hater][1] {
                    if pizza.contains(dislike) {
                        nvm = true;
                        break;
                    }
                }
                if !nvm {
                    // eprintln!("gaining hater: {}", hater,);
                    happy[hater] = true;
                    toggled.push((hater, false));
                    delta += 1;
                }
            }
            for &liker in &ing.likers {
                if !happy[liker] {
                    continue;
                }
                // eprintln!("losing liker: {}", liker,);
                happy[liker] = false;
                toggled.push((liker, true));
                delta -= 1;
            }
        }
        let do_commit = if delta == 0 {
            rand::random()
        } else {
            delta >= 0
        };

        if do_commit && (i != best_score.0) {
            println!(
                "chose ingredient {i} with score {i_score}, best according to heuristic \
                      was {best} with {best_score}",
                i = i,
                i_score = improvability::score(input, &pizza, &happy[..], i).1,
                best = best_score.0,
                best_score = best_score.1
            );
        }

        if do_commit {
            total_delta += delta;
        } else {
            if !pizza.insert(i) {
                pizza.remove(&i);
            }
            for (t, v) in toggled.drain(..) {
                happy[t] = v;
            }
        }
    }
    total_delta
}

fn happy_customers(input: &Input, pizza: &Pizza) -> Vec<bool> {
    let mut ret = vec![true; input.clients.len()];

    for ing in pizza {
        for (i, c) in ret.iter_mut().enumerate() {
            if !*c {
                continue;
            }
            if input.clients[i][1].contains(ing) {
                *c = false;
            }
        }
    }

    for (i, c) in ret.iter_mut().enumerate() {
        if !*c {
            continue;
        }
        for like in &input.clients[i][0] {
            if !pizza.contains(like) {
                *c = false;
            }
        }
    }

    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    stdin().lock().read_to_string(&mut s)?;
    let input = Input::parse(&s);

    // let mut pizza = bake_pizza(&input);
    // let mut pizza: Pizza = (0..input.ings.len()).filter(|_| rand::random()).collect();
    let mut pizza = parse_pizza(&input, include_str!("../outputs/5/d"));
    let mut happy = happy_customers(&input, &pizza);
    let mut score = happy.iter().filter(|&&b| b).count() as isize;
    let mut zeros = 0;

    eprintln!("Initial score: {}", score);

    let mut signals = Signals::new(&[SIGINT])?;
    loop {
        let delta = spice_pizza(&input, &mut pizza, &mut happy);
        score += delta;

        if delta <= 0 {
            zeros += 1;
            if zeros % 1 == 0 {
                eprint!(
                    "\rCtrl+C to exit and print pizza ({}) {}\r",
                    zeros,
                    ".".repeat(1 + (zeros / 100) % 3),
                );
            }
            if signals.pending().count() > 0 {
                break;
            }
        } else {
            eprintln!(
                "\r                                            \r({}) Score: {}",
                zeros, score
            );
            zeros = 0;
        }
    }

    eprintln!("Pizza incoming: ");
    print!("{}", pizza.len());
    for ingredient in pizza {
        print!(" {}", input.ings[ingredient].name);
    }
    println!();

    Ok(())
}
