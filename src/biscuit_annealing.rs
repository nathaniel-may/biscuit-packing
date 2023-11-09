use crate::point::Point;
use argmin::core::{CostFunction, Error, Executor, State};
use argmin::solver::simulatedannealing::{Anneal, SimulatedAnnealing};
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct BiscuitPacking {
    n: usize,
    w: f64,
    l: f64,
    // todo use a better one
    rng: Arc<Mutex<ThreadRng>>,
}

impl BiscuitPacking {
    fn init(&self) -> Vec<Point> {
        // todo start with poisson disc sample
        // https://www.jasondavies.com/poisson-disc/
        let mut out = vec![];
        for _ in 0..self.n {
            let mut rng = self.rng.lock().unwrap();
            let x = rng.sample(Uniform::new(0.0, self.w));
            let y = rng.sample(Uniform::new(0.0, self.l));
            out.push(Point::new(x, y));
        }
        out
    }
}

impl fmt::Display for BiscuitPacking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let biscuits = self.n;
        let width = self.w;
        let length = self.l;
        write!(f, "{biscuits} biscuits in a {width} X {length} pan.")
    }
}

impl Anneal for BiscuitPacking {
    type Param = Vec<Point>;

    type Output = Vec<Point>;

    type Float = f64;

    // example: https://github.com/argmin-rs/argmin/blob/main/argmin/examples/simulatedannealing.rs
    fn anneal(&self, param: &Self::Param, temp: Self::Float) -> Result<Self::Output, Error> {
        let mut param_n = param.clone();
        let mut rng = self.rng.lock().unwrap();
        let idxs = Uniform::from(0..param.len());
        let step = Uniform::new_inclusive(-0.1, 0.1);
        // annealing scales with the number of biscuits in the problem.
        // this is so the same number of runs can be used to solve bigger problems.
        let scale = (self.n as f64 / 7.0).ceil() as usize;
        for _ in 0..(temp.floor() as usize * scale + 1) {
            let idx = rng.sample(idxs);
            let val = rng.sample(step);
            let x = rng.sample(step) >= 0.0;
            if x {
                let x_next = param_n[idx].x + val;
                // don't update if it would cross the bound
                if x_next < self.w && x_next > 0.0 {
                    param_n[idx] = Point {
                        x: x_next,
                        y: param_n[idx].y,
                    };
                }
            } else {
                let y_next = param_n[idx].y + val;
                // don't update if it would cross the bound
                if y_next + val < self.l && y_next > 0.0 {
                    param_n[idx] = Point {
                        x: param_n[idx].x,
                        y: y_next,
                    };
                }
            }
        }
        Ok(param_n)
    }
}

impl CostFunction for BiscuitPacking {
    type Param = Vec<Point>;
    type Output = f64;

    fn cost(&self, biscuit_placement: &Self::Param) -> Result<Self::Output, Error> {
        let mut mins = vec![];

        // for each biscuit, calcualte distances to all other biscuits and pan edges
        for p0 in biscuit_placement {
            // start with the distance of the biscuit to the edges
            let mut p0_ds = vec![self.w - p0.x, p0.x, self.l - p0.y, p0.y];
            for p1 in biscuit_placement {
                if p0 != p1 {
                    p0_ds.push(p0.distance(p1))
                }
            }
            let min = p0_ds.into_iter().reduce(|x, y| x.min(y)).unwrap();
            mins.push(min);
        }

        let min = mins.clone().into_iter().reduce(|x, y| x.min(y)).unwrap();
        let value = self.w.min(self.l) - min;

        Ok(value)
    }
}

pub fn run(biscuits: usize, pan_width: f64, pan_length: f64, runs: u64) -> Vec<Point> {
    let problem = BiscuitPacking {
        n: biscuits,
        l: pan_length,
        w: pan_width,
        rng: Arc::new(Mutex::new(rand::thread_rng())),
    };
    let init = problem.init();
    let solver = SimulatedAnnealing::new(100.0).unwrap();
    let res = Executor::new(problem.clone(), solver)
        .configure(|state| {
            state
                .param(init)
                // stops after this number of iterations (runs in cli)
                .max_iters(runs)
        })
        // run the solver on the defined problem
        .run()
        .unwrap();

    res.state().get_best_param().unwrap().clone()
}
