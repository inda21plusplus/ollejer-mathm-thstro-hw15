use crate::{Input, Pizza};

pub(crate) fn score(
    input: &Input,
    pizza: &Pizza,
    happy: &[bool],
    chosen_ing: usize,
) -> (isize, isize) {
    let new_happy: Vec<bool> = happy
        .iter()
        .enumerate()
        .map(|(p, happy)| {
            *happy
                && input.clients[p][0].contains(&chosen_ing)
                && !input.clients[p][1].contains(&chosen_ing)
        })
        .collect();

    let base = new_happy.iter().filter(|x| **x).count() as isize
        - happy.iter().filter(|x| **x).count() as isize;

    let mut sum = 0;

    for (i_idx, ing) in input.ings.iter().enumerate() {
        if pizza.contains(&i_idx) || i_idx == chosen_ing {
            continue;
        }

        for &p in &ing.likers {
            if new_happy[p] {
                sum += 1;
            }
        }

        for &p in &ing.haters {
            if new_happy[p] {
                sum -= 1;
            }
        }
    }

    (base, sum)
}
