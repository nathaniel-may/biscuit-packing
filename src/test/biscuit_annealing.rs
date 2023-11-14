use crate::biscuit_annealing::BiscuitPacking;
use rand::{rngs::SmallRng, SeedableRng};
use std::sync::{Arc, Mutex};

// todo add prop testing?
#[test]
fn samples_enough_for_init() {
    let rng = Arc::new(Mutex::new(SmallRng::from_entropy()));
    // keeps test deterministic
    let seed = Some(1_u64);
    let mut problem = BiscuitPacking {
        n: 1,
        w: 1.0,
        l: 10.0,
        rng,
    };

    for n in [1, 2, 3, 5, 8, 13, 21, 100] {
        problem.n = n;
        let samples = problem.sample(seed).len();
        assert!(
            samples > n,
            "Not enough samples generated. n:{n}, samples:{samples}"
        );
    }
}
