extern crate rand;

use std::collections::HashSet;
use rand::Rng;
use std::sync::mpsc::{channel, Receiver};
use std::thread::spawn;

pub fn contains(reference: &HashSet<i32>, extraction: &HashSet<i32>) -> bool {
    extraction.is_subset(reference)
}

fn shuffled_subset(mut numbers: Vec<i32>, len: usize) -> HashSet<i32> {
    rand::thread_rng().shuffle(&mut numbers);
    numbers.into_iter().take(len).collect()
}

fn all_numbers(set_dim: usize) -> Vec<i32> {
    (0..set_dim as i32).collect::<Vec<i32>>()
}

fn do_single_test(set_dim: usize, len: usize, extractions: usize) -> bool {
    let reference = shuffled_subset(all_numbers(set_dim), len);
    let extracted = shuffled_subset(all_numbers(set_dim), extractions);
    return contains(&reference, &extracted)
}

fn do_extractions(set_dim: usize, len: usize, extractions: usize, times: usize) -> u32 {
    (0..times).filter(move |_| do_single_test(set_dim, len, extractions)).count() as u32
}

fn spawn_extraction(times: usize) -> Receiver<(usize, u32)> {
    let (sender, receiver) = channel::<(usize, u32)>();
    spawn(move || {
        let wins = do_extractions(100, 50, 2, times);
        sender.send((times, wins))
    });
    receiver
}

fn main() {
    let times = 100000;
    let receivers: Vec<Receiver<(usize, u32)>> = (0..8).map(|_| spawn_extraction(times))
        .collect::<Vec<Receiver<(usize, u32)>>>();
    let mut tot = 0;
    let mut wins = 0;
    for r in receivers {
        if let Ok((t, w)) = r.recv() {
            tot += t;
            wins += w;
        }
    }

    println!("Result [{}] = {} [{}]", tot, (wins as f32)/(tot as f32), wins);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(reference: Vec<i32>, extraction: Vec<i32>, expected: bool) {
        assert_eq!(contains(&reference.iter().cloned().collect(),
                            &extraction.iter().cloned().collect()), expected)
    }

    #[test]
    fn contain_true() {
        do_test(vec![1, 3, 53, 7], vec![3, 7], true);
        do_test(vec![1, 3, 53, 7, 1, 3, 53, 7], vec![3, 7, 7], true);
    }

    #[test]
    fn contain_false() {
        do_test(vec![1, 53, 7], vec![3, 7], false);
        do_test(vec![1, 3, 53], vec![3, 7], false);
        do_test(vec![1, 53], vec![3, 7], false);
    }
}