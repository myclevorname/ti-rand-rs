use rational::Rational;
use std::ops::RangeInclusive;

pub struct Seed {
    seed1: i64,
    seed2: i64,
}

const MOD1: i64 = 2147483563;
const MOD2: i64 = 2147483399;
const MULT1: i64 = 40014;
const MULT2: i64 = 40692;

pub fn generate_seed(seed: &i64) -> Seed {
    let seed: i64 = seed.abs();

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

pub fn rand_rational(seed: &Seed) -> (Seed, Rational) {
    // Not doing the modulus immediately saves a few cycles when you discard
    // the seed. I am not sure this is necessary in Rust, but it was in C.
    let seed1 = seed.seed1 * MULT1;
    let seed2 = (seed.seed2 * MULT2) % MOD2;

    let numerator = (seed1 - seed2) % MOD1;
    let rational = Rational::new(numerator, MOD1);

    let seed = Seed {
        seed1: seed1 % MOD1,
        seed2,
    };

    (seed, rational)
}

pub fn rand(seed: &Seed) -> (Seed, f64) {
    // Not doing the modulus immediately saves a few cycles when you discard
    // the seed. I am not sure this is necessary in Rust, but it was in C.
    let seed1 = seed.seed1 * MULT1;
    let seed2 = seed.seed2 * MULT2 % MOD2;

    let numerator = (seed1 - seed2) % MOD1;
    let result = (numerator as f64) / (MOD1 as f64);

    let seed = Seed {
        seed1: seed1 % MOD1,
        seed2,
    };

    (seed, result)
}

pub fn rand_numerator(seed: &Seed) -> (Seed, i64) {
    // Not doing the modulus immediately saves a few cycles when you discard
    // the seed. I am not sure this is necessary in Rust, but it was in C.
    let seed1 = seed.seed1 * MULT1;
    let seed2 = seed.seed2 * MULT2 % MOD2;

    let numerator = (seed1 - seed2) % MOD1;

    let seed = Seed {
        seed1: seed1 % MOD1,
        seed2,
    };

    (seed, numerator)
}

pub fn rand_int(seed: &Seed, range: &RangeInclusive<i64>) -> (Seed, i64) {
    let (seed, result) = rand(seed);

    let start = *range.end() as f64;
    let end = *range.end() as f64;

    let result = start + (start - end + 1.0) * result as f64;

    (seed, result as i64)
}
