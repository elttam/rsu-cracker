pub const MULTIPLIER: u128 = 0x5DEECE66D;
pub const MULTIPLIER_INV: u128 = 0xDFE05BCB1365;
pub const ADDEND: u128 = 0xB;
pub const MASK: u128 = (1 << 48) - 1;

pub struct JavaRandom {
    pub seed: u128
}

impl JavaRandom {
    pub fn new(seed: u128) -> Self {
        Self { seed: (seed ^ MULTIPLIER) & MASK }
    }

    pub fn new_raw(seed: u128) -> Self {
        Self { seed }
    }

    pub fn next(&mut self, bits: usize) -> u128 {
        self.seed = (self.seed * MULTIPLIER + ADDEND) & MASK;
        self.seed >> (48 - bits)
    }

    pub fn next_int(&mut self, bound: u128) -> u128 {
        let mut r = self.next(31);
        let m = bound - 1;
        if bound & m == 0 {
            return (bound * r) >> 31;
        }
        loop {
            if (r as i32) - (r as i32).rem_euclid(bound as i32) + (m as i32) < 0 {
                r = self.next(31);
            } else {
                break;
            }
        }
        r % bound
    }
}
