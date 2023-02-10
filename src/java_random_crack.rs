use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

use crate::util::{compute_leftm, compute_right_i, recover_seed_from_x1, cartesian_product};
use crate::java_random::JavaRandom;

pub fn check_seed(seed: u128, outputs: &Vec<u128>, modulus: u128) -> bool {
    let mut rand = JavaRandom::new_raw(seed);
    for i in 1..outputs.len() {
        if outputs[i] != rand.next_int(modulus) {
            return false;
        }
    }
    true
}

pub fn recover_state(outputs: Vec<u128>, modulus: u128, m: usize) -> Option<u128> {
    let skips: Vec<usize> = if m == 1 { (0..40).into_iter().collect() } else { vec![0] };
    for skip in skips {
        let left = compute_leftm(&outputs, modulus, m, skip);
        let bar = ProgressBar::new((2u64).pow(17)).with_style(ProgressStyle::with_template("{percent}% {wide_bar:40.cyan/blue} {pos:>7}/{len:7} [{elapsed_precise}<{eta_precise}, {per_sec}]").unwrap());
        let out = (0..(2u128).pow(17)).into_par_iter().find_map_any(|x1_l17_guess| {
            bar.inc(1);
            let mut r_all = vec![];
            for i in 1..(m+1) {
                let r = compute_right_i(x1_l17_guess, i + skip, modulus);
                r_all.push(r);
            }
            for key in cartesian_product(&r_all) {
                let x1_u31_guesses = left.get(&key);
                if x1_u31_guesses.is_none() {
                    continue;
                }
                for x1_u31_guess in x1_u31_guesses.unwrap() {
                    let seed = (x1_u31_guess << 17) | x1_l17_guess;
                    if check_seed(seed, &outputs, modulus) {
                        return Some(seed);
                    }
                }
            }
            None
        });
        if out.is_some() {
            return out;
        }
    }
    None
}

pub fn recover_seed(outputs: Vec<u128>, modulus: u128) -> Option<u128> {
    let m = match modulus {
        0..=6 => 8,
        7..=9 => 7,
        10..=16 => 6,
        17..=32 => 5,
        33..=512 => 3,
        513..=2048 => 2,
        _ => 1

    };
    let x1 = recover_state(outputs, modulus, m);
    if x1.is_none() {
        return None;
    }
    Some(recover_seed_from_x1(x1.unwrap()))
}
