use std::sync::Mutex;

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

lazy_static::lazy_static! {
    static ref RNG: Mutex<SmallRng> = Mutex::new(SmallRng::seed_from_u64(42));
}

pub fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = RNG.lock().unwrap();
    rng.gen()
}

pub fn default<T>() -> T
where
    T: Default,
{
    T::default()
}
