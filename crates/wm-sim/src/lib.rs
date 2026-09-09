pub mod consequences;
pub mod content;
pub mod planning;
pub mod runtime;

use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

pub fn rng(seed: u64, stream: u64) -> ChaCha8Rng {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    rng.set_stream(stream);
    rng
}

// Fixed word consumption and integer arithmetic are part of engine 0.2's contract.
pub fn roll(rng: &mut ChaCha8Rng, upper: u32) -> i32 {
    (rng.next_u32() % upper) as i32
}
pub fn bounded(value: i32) -> i32 {
    value.clamp(0, 100)
}
