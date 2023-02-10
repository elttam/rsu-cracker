use crate::java_random::JavaRandom;

pub struct RandomStringUtils {
    pub random: JavaRandom
}

impl RandomStringUtils {
    pub fn new(seed: u128) -> Self {
        Self { random: JavaRandom::new(seed) }
    }

    pub fn new_raw(seed: u128) -> Self {
        Self { random: JavaRandom::new_raw(seed) }
    }

    pub fn random_alphanumeric(&mut self, count: usize) -> String {
        let start = ' ' as u128;
        let end = 'z' as u128 + 1;
        let gap = end - start;
        let mut out = String::new();
        while out.len() < count {
            let code_point = (self.random.next_int(gap) + start) as u8 as char;
            if code_point.is_alphanumeric() {
                out.push(code_point);
            }
        }
        out
    }

    pub fn random_alphanumeric_first_u128(&mut self) -> u128 {
        let start = ' ' as u128;
        let end = 'z' as u128 + 1;
        let gap = end - start;
        loop {
            let code_point = self.random.next_int(gap) + start;
            if (code_point as u8 as char).is_alphanumeric() {
                return code_point;
            }
        }
    }
}
