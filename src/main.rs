use rayon::prelude::*;
use std::ops::RangeInclusive;
use ti_rand::Seed;

fn main() {
    const RANGE: RangeInclusive<i64> = 1..=1_000_000;
    const TARGET: i64 = 314159;
    let values = (0..=999_999_999)
        .into_par_iter()
        .filter(|x| Seed::new(*x).rand_int(&RANGE) == TARGET)
        .collect::<Vec<i64>>();
    println!("Got seeds: {:?}", values);
}
