use rustc_hash::FxHashMap;
use kdam::tqdm;

use crate::java_random::{MULTIPLIER, MULTIPLIER_INV, ADDEND, MASK};

pub fn multiplier_pow(mut pow: u128) -> u128 {
    let mut base = MULTIPLIER;
    let mut res = 1;
    while pow > 0 {
        if pow & 1 == 1 {
            res = (res * base) & MASK;
        }
        base = (base * base) & MASK;
        pow >>= 1;
    }
    res
}

pub fn compute_left_i(x1_u31_guess: u128, yi_plus_1: u128, i: usize, modulus: u128) -> u128 {
    let mut s = 0;
    for j in 0..i {
        s += multiplier_pow(j as u128);
    }
    s *= ADDEND;
    let r = (((multiplier_pow(i as u128) * (x1_u31_guess << 17) + s) & MASK) >> 17).rem_euclid(modulus);
    if yi_plus_1 > r {
        yi_plus_1 - r
    } else {
        modulus - (r - yi_plus_1)
    }
}

pub fn compute_right_i(x1_l17_guess: u128, i: usize, modulus: u128) -> Vec<u128> {
    let base = ((multiplier_pow(i as u128) * x1_l17_guess) & MASK) >> 17;
    let r1 = base.rem_euclid(modulus);
    let r2 = (base as i128 - 2i128.pow(31)).rem_euclid(modulus as i128) as u128;
    vec![r1 as u128, r2 as u128, (r1 + 1).rem_euclid(modulus) as u128, (r2 + 1).rem_euclid(modulus) as u128]
}

pub fn compute_left(outputs: &Vec<u128>, modulus: u128, skips: (usize, usize, usize)) -> FxHashMap<(u128, u128, u128), Vec<u128>> {
    let mut left: FxHashMap<(u128, u128, u128), Vec<u128>> = FxHashMap::default();
    for k1_guess in tqdm!(0..((2u128).pow(31) / modulus)) {
        let x1_u31_guess = outputs[0] + modulus * k1_guess;
        let l1 = compute_left_i(x1_u31_guess, outputs[1], 1 + skips.0, modulus);
        let l2 = compute_left_i(x1_u31_guess, outputs[2], 2 + skips.0 + skips.1, modulus);
        let l3 = compute_left_i(x1_u31_guess, outputs[3], 3 + skips.0 + skips.1 + skips.2, modulus);
        let key = (l1, l2, l3);
        left.entry(key).or_insert(vec![]).push(x1_u31_guess);
    }
    left
}

pub fn compute_leftm(outputs: &Vec<u128>, modulus: u128, m: usize, skip: usize) -> FxHashMap<Vec<u128> , Vec<u128>> {
    let mut left: FxHashMap<Vec<u128>, Vec<u128>> = FxHashMap::default();
    for k1_guess in tqdm!(0..((2u128).pow(31) / modulus)) {
        let x1_u31_guess = outputs[0] + modulus * k1_guess;
        let mut key = vec![];
        for i in 1..(m+1) {
            let l = compute_left_i(x1_u31_guess, outputs[i], i + skip, modulus);
            key.push(l);
        }
        left.entry(key).or_insert(vec![]).push(x1_u31_guess);
    }
    left
}

pub fn recover_seed_from_x1(x1: u128) -> u128 {
    ((((x1 - ADDEND).rem_euclid(MASK + 1) * MULTIPLIER_INV) & MASK) ^ MULTIPLIER) & MASK
}

// from https://rosettacode.org/wiki/Cartesian_product_of_two_or_more_lists
pub fn cartesian_product(lists: &Vec<Vec<u128>>) -> Vec<Vec<u128>> {
    let mut res = vec![];

    let mut list_iter = lists.iter();
    if let Some(first_list) = list_iter.next() {
        for &i in first_list {
            res.push(vec![i]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for &el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}
