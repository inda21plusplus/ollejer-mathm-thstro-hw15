use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{stdin, Read},
};

use signal_hook::{consts::SIGINT, iterator::Signals};

#[derive(Debug, Clone)]
struct Ingredient<'i> {
    name: &'i str,
    likers: Vec<usize>,
    haters: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Input<'i> {
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

fn season_pizza(
    input: &Input,
    mut pizza: Pizza,
    mut happy: Vec<bool>,
) -> (Pizza, Vec<bool>, isize, bool) {
    let mut total_delta = 0;
    let mut found_any_change = false;
    for (i, ing) in input.ings.iter().enumerate() {
        if rand::random::<usize>() < usize::MAX / 2 {
            continue;
        }
        let mut toggled = vec![];
        let mut delta = 0isize;
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
                    toggled.push(liker);
                    delta += 1;
                }
            }
            for &hater in &ing.haters {
                if !happy[hater] {
                    continue;
                }
                // eprintln!("losing hater: {}", hater,);
                happy[hater] = false;
                toggled.push(hater);
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
                    toggled.push(hater);
                    delta += 1;
                }
            }
            for &liker in &ing.likers {
                if !happy[liker] {
                    continue;
                }
                // eprintln!("losing liker: {}", liker,);
                happy[liker] = false;
                toggled.push(liker);
                delta -= 1;
            }
        }
        let do_commit = if delta == 0 {
            rand::random()
        } else {
            delta >= 0
        };
        if do_commit {
            total_delta += delta;
            found_any_change = true;
        } else {
            if !pizza.insert(i) {
                pizza.remove(&i);
            }
            for t in toggled.drain(..) {
                happy[t] = !happy[t];
            }
        }
    }
    (pizza, happy, total_delta, found_any_change)
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

    let mut pizza = bake_pizza(&input);
    let mut happy = happy_customers(&input, &pizza);
    let mut score = happy.iter().filter(|&&b| b).count() as isize;
    let mut zeros = 0;

    let mut signals = Signals::new(&[SIGINT])?;
    loop {
        let (p, h, delta, any_change) = season_pizza(&input, pizza, happy);
        pizza = p;
        happy = h;
        score += delta;
        if !any_change {
            break;
        }

        if delta == 0 {
            zeros += 1;
            if zeros % 100 == 0 {
                eprint!(
                    "\r Ctrl+C to exit and print pizza ({}) {}   ",
                    zeros,
                    ".".repeat(1 + (zeros / 100) % 3),
                );
            }
            if signals.pending().count() > 0 {
                break;
            }
        } else {
            eprintln!("\r({}) Score: {}", zeros, score);
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
