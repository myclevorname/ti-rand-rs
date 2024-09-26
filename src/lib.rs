use std::ops::RangeInclusive;


pub struct Seed {
    seed1: i64,
    seed2: i64,
}

const MOD1: i64 = 2147483563;
const MOD2: i64 = 2147483399;
const MULT1: i64 = 40014;
const MULT2: i64 = 40692;

impl Default for Seed {
    fn default() -> Seed {
        Seed::new(0)
    }
}

impl Seed {
    pub fn new(seed: i64) -> Seed {
        let seed = seed.abs();
        if seed == 0 {
            Seed {
                seed1: 12345,
                seed2: 67890,
            }
        } else {
            Seed {
                seed1: (MULT1 * seed) % MOD1,
                seed2: seed % MOD2,
            }
        }
    }

    pub fn rand_numerator(&mut self) -> i64 {
        let seed1 = self.seed1 * MULT1;
        let seed2 = (self.seed2 * MULT2) % MOD2;
        *self = Seed { seed1: seed1 % MOD1, seed2 };

        (seed1 - seed2) % MOD1
    }

    pub fn rand(&mut self) -> f64 {
        let numerator = Seed::rand_numerator(self) as f64;
        numerator / (MOD1 as f64)
    }

    pub fn rand_int(&mut self, range: &RangeInclusive<i64>) -> i64 {
        let result = Seed::rand_numerator(self);

        let start = *range.start();
        let end = *range.end();

        start + ((start - end + 1) * result) / MOD1
    }
}
