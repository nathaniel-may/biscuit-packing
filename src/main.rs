use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::simulatedannealing::{Anneal, SimulatedAnnealing};
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

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
        for _ in 0..(temp.floor() as u64 + 1) {
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
        // todo remove print
        // println!("biscuit_placement {biscuit_placement:?}");
        // println!("mins {mins:?}");
        // println!("min {min}");
        // println!("value {value}");
        // println!("-------------");

        Ok(value)
    }
}

fn main() {
    let problem = BiscuitPacking {
        n: 2,
        l: 1.0,
        w: 1.0,
        rng: Arc::new(Mutex::new(rand::thread_rng())),
    };
    let init = problem.init();
    let solver = SimulatedAnnealing::new(100.0).unwrap();
    let res = Executor::new(problem.clone(), solver)
        .configure(|state| {
            state
                // Set initial parameters (depending on the solver,
                // this may be required)
                .param(init)
                // Set maximum iterations to 10
                // (optional, set to `std::u64::MAX` if not provided)
                .max_iters(100000)
        })
        // run the solver on the defined problem
        .run()
        .unwrap();

    println!("{}", problem);
    println!("{}", res);
}
