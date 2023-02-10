use rayon::prelude::*;
use rayon::iter::ParallelIterator;
use itertools::iproduct;

use crate::util::{compute_left, compute_right_i, recover_seed_from_x1};
use crate::random_string_utils::RandomStringUtils;

pub fn check_seed(seed: u128, outputs: &Vec<u128>) -> bool {
    let mut rsu = RandomStringUtils::new_raw(seed);
    for i in 1..(outputs.len().min(9)) {
        if outputs[i] != rsu.random_alphanumeric_first_u128() - 32 {
            return false;
        }
    }
    true
}

pub fn recover_state(outputs: Vec<u128>) -> Option<u128> {
    let mut all_skips: Vec<(usize, usize, usize)> = Vec::new();
    for (s1, s2, s3) in iproduct!(0..8, 0..8, 0..8) {
        all_skips.push((s1, s2, s3));
    }
    all_skips.sort_by_key(|x| x.0 + x.1 + x.2);
    all_skips.iter().find_map(|&skips| {
        eprintln!("[*] Attempting with skips = {:?}", skips);
        let left = compute_left(&outputs, 91, skips);
        eprintln!("");
        (0..(2u128).pow(17)).into_par_iter().find_map_any(|x1_l17_guess| {
            let r1_all = compute_right_i(x1_l17_guess, 1 + skips.0, 91);
            let r2_all = compute_right_i(x1_l17_guess, 2 + skips.0 + skips.1, 91);
            let r3_all = compute_right_i(x1_l17_guess, 3 + skips.0 + skips.1 + skips.2, 91);
            for (r1, r2, r3) in iproduct!(r1_all, r2_all, r3_all) {
                let x1_u31_guesses = left.get(&(r1, r2, r3));
                if x1_u31_guesses.is_none() {
                    continue;
                }
                for x1_u31_guess in x1_u31_guesses.unwrap() {
                    let seed = (x1_u31_guess << 17) | x1_l17_guess;
                    if check_seed(seed, &outputs) {
                        return Some(seed);
                    }
                }
            }
            None
        })
    })
}

pub fn recover_seed(outputs: Vec<u128>) -> Option<u128> {
    let x1 = recover_state(outputs);
    if x1.is_none() {
        return None;
    }
    Some(recover_seed_from_x1(x1.unwrap()))
}
