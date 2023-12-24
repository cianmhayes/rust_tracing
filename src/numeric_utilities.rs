use rand::distributions::Standard;
use rand::prelude::*;

pub fn linear_to_gamma(linear: f32) -> f32 {
    linear.sqrt()
}

pub fn get_rand_float() -> f32 {
    rand::thread_rng().sample(Standard)
}
